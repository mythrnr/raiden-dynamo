use quote::*;

pub(crate) fn expand_scan(
    struct_name: &proc_macro2::Ident,
    fields: &syn::FieldsNamed,
    rename_all_type: crate::rename::RenameAllType,
) -> proc_macro2::TokenStream {
    let trait_name = format_ident!("{}Scan", struct_name);
    let client_name = format_ident!("{}Client", struct_name);
    let builder_name = format_ident!("{}ScanBuilder", struct_name);

    let filter_expression_token_name = format_ident!("{}FilterExpressionToken", struct_name);
    let from_item = super::expand_attr_to_item(format_ident!("res_item"), fields, rename_all_type);
    let api_call_token = super::api_call_token!("scan");
    let (call_inner_run, inner_run_args) = if cfg!(feature = "tracing") {
        (
            quote! { #builder_name::inner_run(input.table_name.clone(), client, input).await },
            quote! { table_name: String, },
        )
    } else {
        (
            quote! { #builder_name::inner_run(client, input).await },
            quote! {},
        )
    };

    quote! {
        pub trait #trait_name {
            fn scan(&self) -> #builder_name;
        }

        pub struct #builder_name<'a> {
            pub client: &'a ::raiden::DynamoDbClient,
            pub input: ::raiden::ScanInput,
            pub policy: ::raiden::Policy,
            pub condition: &'a ::raiden::retry::RetryCondition,
            pub next_token: Option<::raiden::NextToken>,
            pub limit: Option<i64>
        }

        impl #trait_name for #client_name {
            #![allow(clippy::field_reassign_with_default)]
            fn scan(&self) -> #builder_name {
                let mut input = ::raiden::ScanInput::default();
                input.table_name = self.table_name();
                input.projection_expression = self.projection_expression.clone();
                input.expression_attribute_names = self.attribute_names.clone();
                #builder_name {
                    client: &self.client,
                    input,
                    policy: self.retry_condition.strategy.policy(),
                    condition: &self.retry_condition,
                    next_token: None,
                    limit: None,
                }
            }
        }

        impl<'a> #builder_name<'a> {
            pub fn index(mut self, index: impl Into<String>) -> Self {
                self.input.index_name = Some(index.into());
                self
            }

            pub fn consistent(mut self) -> Self {
                self.input.consistent_read = Some(true);
                self
            }

            pub fn filter(mut self, cond: impl ::raiden::filter_expression::FilterExpressionBuilder<#filter_expression_token_name>) -> Self {
                let (cond_str, attr_names, attr_values) = cond.build();
                if !attr_values.is_empty() {
                    if let Some(v) = self.input.expression_attribute_values {
                        self.input.expression_attribute_values = Some(::raiden::merge_map(attr_values, v));
                    } else {
                        self.input.expression_attribute_values = Some(attr_values);
                    }
                }
                self.input.filter_expression = Some(cond_str);
                self
            }

            pub fn next_token(mut self, token: ::raiden::NextToken) -> Self {
                self.next_token = Some(token);
                self
            }

            pub fn limit(mut self, limit: usize) -> Self {
                self.limit = Some(limit as i64);
                self
            }

            pub async fn run(mut self) -> Result<::raiden::scan::ScanOutput<#struct_name>, ::raiden::RaidenError> {
                if let Some(token) = self.next_token {
                    self.input.exclusive_start_key = Some(token.into_attr_values()?);
                }

                let mut consumed_capacity = None;
                let mut count = 0;
                let mut items: Vec<#struct_name> = vec![];
                let mut scanned_count = 0;

                loop {
                    if let Some(limit) = self.limit {
                        self.input.limit = Some(limit);
                    }

                    let res = {
                        let policy: ::raiden::RetryPolicy = self.policy.clone().into();
                        let client = self.client;
                        let input = self.input.clone();
                        policy.retry_if(move || {
                            let client = client.clone();
                            let input = input.clone();
                            async { #call_inner_run }
                        }, self.condition).await?
                    };

                    if let Some(res_items) = res.items {
                        for res_item in res_items.into_iter() {
                            let mut res_item = res_item;
                            items.push(#struct_name {
                                #(#from_item)*
                            })
                        }
                    };

                    let current_count = res.count.unwrap_or(0);
                    count += current_count;
                    scanned_count += res.scanned_count.unwrap_or(0);
                    if let Some(c) = res.consumed_capacity {
                        consumed_capacity = if let Some(d) = consumed_capacity {
                            Some(::raiden::ops::consumed_capacity::extend_consumed_capacity(d, c))
                        } else {
                            Some(c)
                        };
                    }

                    let mut has_next = true;
                    if let Some(limit) = self.limit {
                        has_next = limit - current_count > 0;
                        self.limit = Some(limit - current_count);
                    }

                    if res.last_evaluated_key.is_none() || !has_next {
                        return Ok(::raiden::scan::ScanOutput {
                            consumed_capacity,
                            count: Some(count),
                            items,
                            last_evaluated_key: res.last_evaluated_key,
                            scanned_count: Some(scanned_count),
                        })
                    }

                    self.input.exclusive_start_key = res.last_evaluated_key;
                }
            }

            async fn inner_run(
                #inner_run_args
                client: ::raiden::DynamoDbClient,
                input: ::raiden::ScanInput,
            ) -> Result<::raiden::ScanOutput, ::raiden::RaidenError> {
                Ok(#api_call_token?)
            }
        }
    }
}
