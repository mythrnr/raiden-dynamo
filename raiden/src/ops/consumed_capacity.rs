#[cfg(any(feature = "rusoto", feature = "rusoto_rustls"))]
use crate::{Capacity, ConsumedCapacity};

#[cfg(feature = "aws-sdk")]
use crate::aws_sdk::types::{Capacity, ConsumedCapacity};

#[cfg(any(feature = "rusoto", feature = "rusoto_rustls"))]
pub fn extend_consumed_capacity(a: ConsumedCapacity, b: ConsumedCapacity) -> ConsumedCapacity {
    let capacity_units = if let Some(c) = a.capacity_units {
        Some(c + b.capacity_units.unwrap_or(0_f64))
    } else {
        b.capacity_units
    };

    let global_secondary_indexes = if let Some(c) = a.global_secondary_indexes {
        let mut gsi = std::collections::HashMap::new();
        let mut keys = c.keys().collect::<std::collections::BTreeSet<_>>();
        let d = b.global_secondary_indexes.unwrap_or_default();

        keys.extend(d.keys());

        for k in keys {
            match (c.get(k), d.get(k)) {
                (Some(c), Some(d)) => {
                    gsi.insert(k.clone(), extend_capacity(c.clone(), d.clone()));
                }
                (Some(c), None) => {
                    gsi.insert(k.clone(), c.clone());
                }
                (None, Some(d)) => {
                    gsi.insert(k.clone(), d.clone());
                }
                _ => {}
            };
        }

        Some(gsi)
    } else {
        b.global_secondary_indexes
    };

    let local_secondary_indexes = if let Some(c) = a.local_secondary_indexes {
        let mut lsi = std::collections::HashMap::new();
        let mut keys = c.keys().collect::<std::collections::BTreeSet<_>>();
        let d = b.local_secondary_indexes.unwrap_or_default();

        keys.extend(d.keys());

        for k in keys {
            match (c.get(k), d.get(k)) {
                (Some(c), Some(d)) => {
                    lsi.insert(k.clone(), extend_capacity(c.clone(), d.clone()));
                }
                (Some(c), None) => {
                    lsi.insert(k.clone(), c.clone());
                }
                (None, Some(d)) => {
                    lsi.insert(k.clone(), d.clone());
                }
                _ => {}
            };
        }

        Some(lsi)
    } else {
        b.local_secondary_indexes
    };

    let table = if let Some(c) = a.table {
        if let Some(d) = b.table {
            Some(extend_capacity(c, d))
        } else {
            Some(c)
        }
    } else {
        b.table
    };

    let read_capacity_units = if let Some(c) = a.read_capacity_units {
        Some(c + b.read_capacity_units.unwrap_or(0_f64))
    } else {
        b.read_capacity_units
    };

    let write_capacity_units = if let Some(c) = a.write_capacity_units {
        Some(c + b.write_capacity_units.unwrap_or(0_f64))
    } else {
        b.write_capacity_units
    };

    ConsumedCapacity {
        capacity_units,
        global_secondary_indexes,
        local_secondary_indexes,
        read_capacity_units,
        table,
        table_name: a.table_name.or(b.table_name),
        write_capacity_units,
    }
}

#[cfg(any(feature = "rusoto", feature = "rusoto_rustls"))]
fn extend_capacity(a: Capacity, b: Capacity) -> Capacity {
    let capacity_units = if let Some(c) = a.capacity_units {
        Some(c + b.capacity_units.unwrap_or(0_f64))
    } else {
        b.capacity_units
    };

    let read_capacity_units = if let Some(c) = a.read_capacity_units {
        Some(c + b.read_capacity_units.unwrap_or(0_f64))
    } else {
        b.read_capacity_units
    };

    let write_capacity_units = if let Some(c) = a.write_capacity_units {
        Some(c + b.write_capacity_units.unwrap_or(0_f64))
    } else {
        b.write_capacity_units
    };

    Capacity {
        capacity_units,
        read_capacity_units,
        write_capacity_units,
    }
}

#[cfg(feature = "aws-sdk")]
pub fn extend_consumed_capacity(a: ConsumedCapacity, b: ConsumedCapacity) -> ConsumedCapacity {
    ConsumedCapacity::builder()
        .set_capacity_units(if let Some(c) = a.capacity_units {
            Some(c + b.capacity_units.unwrap_or(0_f64))
        } else {
            b.capacity_units
        })
        .set_global_secondary_indexes(if let Some(c) = a.global_secondary_indexes {
            let mut gsi = std::collections::HashMap::new();
            let mut keys = c.keys().collect::<std::collections::BTreeSet<_>>();
            let d = b.global_secondary_indexes.unwrap_or_default();

            keys.extend(d.keys());

            for k in keys {
                match (c.get(k), d.get(k)) {
                    (Some(c), Some(d)) => {
                        gsi.insert(k.clone(), extend_capacity(c.clone(), d.clone()));
                    }
                    (Some(c), None) => {
                        gsi.insert(k.clone(), c.clone());
                    }
                    (None, Some(d)) => {
                        gsi.insert(k.clone(), d.clone());
                    }
                    _ => {}
                };
            }

            Some(gsi)
        } else {
            b.global_secondary_indexes
        })
        .set_local_secondary_indexes(if let Some(c) = a.local_secondary_indexes {
            let mut lsi = std::collections::HashMap::new();
            let mut keys = c.keys().collect::<std::collections::BTreeSet<_>>();
            let d = b.local_secondary_indexes.unwrap_or_default();

            keys.extend(d.keys());

            for k in keys {
                match (c.get(k), d.get(k)) {
                    (Some(c), Some(d)) => {
                        lsi.insert(k.clone(), extend_capacity(c.clone(), d.clone()));
                    }
                    (Some(c), None) => {
                        lsi.insert(k.clone(), c.clone());
                    }
                    (None, Some(d)) => {
                        lsi.insert(k.clone(), d.clone());
                    }
                    _ => {}
                };
            }

            Some(lsi)
        } else {
            b.local_secondary_indexes
        })
        .set_read_capacity_units(if let Some(c) = a.read_capacity_units {
            Some(c + b.read_capacity_units.unwrap_or(0_f64))
        } else {
            b.read_capacity_units
        })
        .set_table(if let Some(c) = a.table {
            if let Some(d) = b.table {
                Some(extend_capacity(c, d))
            } else {
                Some(c)
            }
        } else {
            b.table
        })
        .set_table_name(a.table_name.or(b.table_name))
        .set_write_capacity_units(if let Some(c) = a.write_capacity_units {
            Some(c + b.write_capacity_units.unwrap_or(0_f64))
        } else {
            b.write_capacity_units
        })
        .build()
}

#[cfg(feature = "aws-sdk")]
fn extend_capacity(a: Capacity, b: Capacity) -> Capacity {
    Capacity::builder()
        .set_capacity_units(if let Some(c) = a.capacity_units {
            Some(c + b.capacity_units.unwrap_or(0_f64))
        } else {
            b.capacity_units
        })
        .set_read_capacity_units(if let Some(c) = a.read_capacity_units {
            Some(c + b.read_capacity_units.unwrap_or(0_f64))
        } else {
            b.read_capacity_units
        })
        .set_write_capacity_units(if let Some(c) = a.write_capacity_units {
            Some(c + b.write_capacity_units.unwrap_or(0_f64))
        } else {
            b.write_capacity_units
        })
        .build()
}

#[cfg(any(feature = "rusoto", feature = "rusoto_rustls"))]
#[cfg(test)]
mod tests {
    use super::{extend_capacity, extend_consumed_capacity, Capacity, ConsumedCapacity};
    use std::{collections::HashMap, iter::FromIterator};

    #[test]
    fn test_extend_capacity() {
        let inputs = vec![
            (
                Capacity::default(),
                Capacity::default(),
                Capacity::default(),
            ),
            (
                Capacity {
                    capacity_units: 1_f64.into(),
                    read_capacity_units: 1_f64.into(),
                    write_capacity_units: 1_f64.into(),
                },
                Capacity::default(),
                Capacity {
                    capacity_units: 1_f64.into(),
                    read_capacity_units: 1_f64.into(),
                    write_capacity_units: 1_f64.into(),
                },
            ),
            (
                Capacity::default(),
                Capacity {
                    capacity_units: 1_f64.into(),
                    read_capacity_units: 1_f64.into(),
                    write_capacity_units: 1_f64.into(),
                },
                Capacity {
                    capacity_units: 1_f64.into(),
                    read_capacity_units: 1_f64.into(),
                    write_capacity_units: 1_f64.into(),
                },
            ),
            (
                Capacity {
                    capacity_units: 1_f64.into(),
                    read_capacity_units: 1_f64.into(),
                    write_capacity_units: 1_f64.into(),
                },
                Capacity {
                    capacity_units: 1_f64.into(),
                    read_capacity_units: 1_f64.into(),
                    write_capacity_units: 1_f64.into(),
                },
                Capacity {
                    capacity_units: 2_f64.into(),
                    read_capacity_units: 2_f64.into(),
                    write_capacity_units: 2_f64.into(),
                },
            ),
        ];

        for (a, b, c) in inputs {
            assert_eq!(extend_capacity(a, b), c);
        }
    }

    #[test]
    fn test_extend_consumed_capacity() {
        let inputs = vec![
            (
                ConsumedCapacity::default(),
                ConsumedCapacity::default(),
                ConsumedCapacity::default(),
            ),
            (
                ConsumedCapacity {
                    capacity_units: 1_f64.into(),
                    local_secondary_indexes: HashMap::from_iter([(
                        "localIndex".to_owned(),
                        Capacity {
                            capacity_units: 1_f64.into(),
                            read_capacity_units: 1_f64.into(),
                            write_capacity_units: 1_f64.into(),
                        },
                    )])
                    .into(),
                    global_secondary_indexes: HashMap::from_iter([(
                        "globalIndex".to_owned(),
                        Capacity {
                            capacity_units: 1_f64.into(),
                            read_capacity_units: 1_f64.into(),
                            write_capacity_units: 1_f64.into(),
                        },
                    )])
                    .into(),
                    read_capacity_units: 1_f64.into(),
                    table: Capacity {
                        capacity_units: 1_f64.into(),
                        read_capacity_units: 1_f64.into(),
                        write_capacity_units: 1_f64.into(),
                    }
                    .into(),
                    table_name: "Table".to_owned().into(),
                    write_capacity_units: 1_f64.into(),
                },
                ConsumedCapacity::default(),
                ConsumedCapacity {
                    capacity_units: 1_f64.into(),
                    local_secondary_indexes: HashMap::from_iter([(
                        "localIndex".to_owned(),
                        Capacity {
                            capacity_units: 1_f64.into(),
                            read_capacity_units: 1_f64.into(),
                            write_capacity_units: 1_f64.into(),
                        },
                    )])
                    .into(),
                    global_secondary_indexes: HashMap::from_iter([(
                        "globalIndex".to_owned(),
                        Capacity {
                            capacity_units: 1_f64.into(),
                            read_capacity_units: 1_f64.into(),
                            write_capacity_units: 1_f64.into(),
                        },
                    )])
                    .into(),
                    read_capacity_units: 1_f64.into(),
                    table: Capacity {
                        capacity_units: 1_f64.into(),
                        read_capacity_units: 1_f64.into(),
                        write_capacity_units: 1_f64.into(),
                    }
                    .into(),
                    table_name: "Table".to_owned().into(),
                    write_capacity_units: 1_f64.into(),
                },
            ),
            (
                ConsumedCapacity::default(),
                ConsumedCapacity {
                    capacity_units: 1_f64.into(),
                    local_secondary_indexes: HashMap::from_iter([(
                        "localIndex".to_owned(),
                        Capacity {
                            capacity_units: 1_f64.into(),
                            read_capacity_units: 1_f64.into(),
                            write_capacity_units: 1_f64.into(),
                        },
                    )])
                    .into(),
                    global_secondary_indexes: HashMap::from_iter([(
                        "globalIndex".to_owned(),
                        Capacity {
                            capacity_units: 1_f64.into(),
                            read_capacity_units: 1_f64.into(),
                            write_capacity_units: 1_f64.into(),
                        },
                    )])
                    .into(),
                    read_capacity_units: 1_f64.into(),
                    table: Capacity {
                        capacity_units: 1_f64.into(),
                        read_capacity_units: 1_f64.into(),
                        write_capacity_units: 1_f64.into(),
                    }
                    .into(),
                    table_name: "Table".to_owned().into(),
                    write_capacity_units: 1_f64.into(),
                },
                ConsumedCapacity {
                    capacity_units: 1_f64.into(),
                    local_secondary_indexes: HashMap::from_iter([(
                        "localIndex".to_owned(),
                        Capacity {
                            capacity_units: 1_f64.into(),
                            read_capacity_units: 1_f64.into(),
                            write_capacity_units: 1_f64.into(),
                        },
                    )])
                    .into(),
                    global_secondary_indexes: HashMap::from_iter([(
                        "globalIndex".to_owned(),
                        Capacity {
                            capacity_units: 1_f64.into(),
                            read_capacity_units: 1_f64.into(),
                            write_capacity_units: 1_f64.into(),
                        },
                    )])
                    .into(),
                    read_capacity_units: 1_f64.into(),
                    table: Capacity {
                        capacity_units: 1_f64.into(),
                        read_capacity_units: 1_f64.into(),
                        write_capacity_units: 1_f64.into(),
                    }
                    .into(),
                    table_name: "Table".to_owned().into(),
                    write_capacity_units: 1_f64.into(),
                },
            ),
            (
                ConsumedCapacity {
                    capacity_units: 1_f64.into(),
                    local_secondary_indexes: HashMap::from_iter([(
                        "localIndex".to_owned(),
                        Capacity {
                            capacity_units: 1_f64.into(),
                            read_capacity_units: 1_f64.into(),
                            write_capacity_units: 1_f64.into(),
                        },
                    )])
                    .into(),
                    global_secondary_indexes: HashMap::from_iter([(
                        "globalIndex".to_owned(),
                        Capacity {
                            capacity_units: 1_f64.into(),
                            read_capacity_units: 1_f64.into(),
                            write_capacity_units: 1_f64.into(),
                        },
                    )])
                    .into(),
                    read_capacity_units: 1_f64.into(),
                    table: Capacity {
                        capacity_units: 1_f64.into(),
                        read_capacity_units: 1_f64.into(),
                        write_capacity_units: 1_f64.into(),
                    }
                    .into(),
                    table_name: "Table".to_owned().into(),
                    write_capacity_units: 1_f64.into(),
                },
                ConsumedCapacity {
                    capacity_units: 1_f64.into(),
                    local_secondary_indexes: HashMap::from_iter([(
                        "localIndex".to_owned(),
                        Capacity {
                            capacity_units: 1_f64.into(),
                            read_capacity_units: 1_f64.into(),
                            write_capacity_units: 1_f64.into(),
                        },
                    )])
                    .into(),
                    global_secondary_indexes: HashMap::from_iter([(
                        "globalIndex".to_owned(),
                        Capacity {
                            capacity_units: 1_f64.into(),
                            read_capacity_units: 1_f64.into(),
                            write_capacity_units: 1_f64.into(),
                        },
                    )])
                    .into(),
                    read_capacity_units: 1_f64.into(),
                    table: Capacity {
                        capacity_units: 1_f64.into(),
                        read_capacity_units: 1_f64.into(),
                        write_capacity_units: 1_f64.into(),
                    }
                    .into(),
                    table_name: "Table".to_owned().into(),
                    write_capacity_units: 1_f64.into(),
                },
                ConsumedCapacity {
                    capacity_units: 2_f64.into(),
                    local_secondary_indexes: HashMap::from_iter([(
                        "localIndex".to_owned(),
                        Capacity {
                            capacity_units: 2_f64.into(),
                            read_capacity_units: 2_f64.into(),
                            write_capacity_units: 2_f64.into(),
                        },
                    )])
                    .into(),
                    global_secondary_indexes: HashMap::from_iter([(
                        "globalIndex".to_owned(),
                        Capacity {
                            capacity_units: 2_f64.into(),
                            read_capacity_units: 2_f64.into(),
                            write_capacity_units: 2_f64.into(),
                        },
                    )])
                    .into(),
                    read_capacity_units: 2_f64.into(),
                    table: Capacity {
                        capacity_units: 2_f64.into(),
                        read_capacity_units: 2_f64.into(),
                        write_capacity_units: 2_f64.into(),
                    }
                    .into(),
                    table_name: "Table".to_owned().into(),
                    write_capacity_units: 2_f64.into(),
                },
            ),
        ];

        for (a, b, c) in inputs {
            assert_eq!(extend_consumed_capacity(a, b), c);
        }
    }
}

#[cfg(feature = "aws-sdk")]
#[cfg(test)]
mod tests {
    use std::{collections::HashMap, iter::FromIterator};

    use crate::{
        aws_sdk::types::{Capacity, ConsumedCapacity},
        consumed_capacity::{extend_capacity, extend_consumed_capacity},
    };

    #[test]
    fn test_extend_capacity() {
        let inputs = vec![
            (
                Capacity::builder().build(),
                Capacity::builder().build(),
                Capacity::builder().build(),
            ),
            (
                Capacity::builder()
                    .capacity_units(1_f64)
                    .read_capacity_units(1_f64)
                    .write_capacity_units(1_f64)
                    .build(),
                Capacity::builder().build(),
                Capacity::builder()
                    .capacity_units(1_f64)
                    .read_capacity_units(1_f64)
                    .write_capacity_units(1_f64)
                    .build(),
            ),
            (
                Capacity::builder().build(),
                Capacity::builder()
                    .capacity_units(1_f64)
                    .read_capacity_units(1_f64)
                    .write_capacity_units(1_f64)
                    .build(),
                Capacity::builder()
                    .capacity_units(1_f64)
                    .read_capacity_units(1_f64)
                    .write_capacity_units(1_f64)
                    .build(),
            ),
            (
                Capacity::builder()
                    .capacity_units(1_f64)
                    .read_capacity_units(1_f64)
                    .write_capacity_units(1_f64)
                    .build(),
                Capacity::builder()
                    .capacity_units(1_f64)
                    .read_capacity_units(1_f64)
                    .write_capacity_units(1_f64)
                    .build(),
                Capacity::builder()
                    .capacity_units(2_f64)
                    .read_capacity_units(2_f64)
                    .write_capacity_units(2_f64)
                    .build(),
            ),
        ];

        for (a, b, c) in inputs {
            assert_eq!(extend_capacity(a, b), c);
        }
    }

    #[test]
    fn test_extend_consumed_capacity() {
        let inputs = vec![
            (
                ConsumedCapacity::builder().build(),
                ConsumedCapacity::builder().build(),
                ConsumedCapacity::builder().build(),
            ),
            (
                ConsumedCapacity::builder()
                    .capacity_units(1_f64)
                    .set_local_secondary_indexes(
                        HashMap::from_iter([(
                            "localIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(1_f64)
                                .read_capacity_units(1_f64)
                                .write_capacity_units(1_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .set_global_secondary_indexes(
                        HashMap::from_iter([(
                            "globalIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(1_f64)
                                .read_capacity_units(1_f64)
                                .write_capacity_units(1_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .read_capacity_units(1_f64)
                    .table(
                        Capacity::builder()
                            .capacity_units(1_f64)
                            .read_capacity_units(1_f64)
                            .write_capacity_units(1_f64)
                            .build(),
                    )
                    .table_name("Table")
                    .write_capacity_units(1_f64)
                    .build(),
                ConsumedCapacity::builder().build(),
                ConsumedCapacity::builder()
                    .capacity_units(1_f64)
                    .set_local_secondary_indexes(
                        HashMap::from_iter([(
                            "localIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(1_f64)
                                .read_capacity_units(1_f64)
                                .write_capacity_units(1_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .set_global_secondary_indexes(
                        HashMap::from_iter([(
                            "globalIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(1_f64)
                                .read_capacity_units(1_f64)
                                .write_capacity_units(1_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .read_capacity_units(1_f64)
                    .table(
                        Capacity::builder()
                            .capacity_units(1_f64)
                            .read_capacity_units(1_f64)
                            .write_capacity_units(1_f64)
                            .build(),
                    )
                    .table_name("Table")
                    .write_capacity_units(1_f64)
                    .build(),
            ),
            (
                ConsumedCapacity::builder().build(),
                ConsumedCapacity::builder()
                    .capacity_units(1_f64)
                    .set_local_secondary_indexes(
                        HashMap::from_iter([(
                            "localIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(1_f64)
                                .read_capacity_units(1_f64)
                                .write_capacity_units(1_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .set_global_secondary_indexes(
                        HashMap::from_iter([(
                            "globalIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(1_f64)
                                .read_capacity_units(1_f64)
                                .write_capacity_units(1_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .read_capacity_units(1_f64)
                    .table(
                        Capacity::builder()
                            .capacity_units(1_f64)
                            .read_capacity_units(1_f64)
                            .write_capacity_units(1_f64)
                            .build(),
                    )
                    .table_name("Table")
                    .write_capacity_units(1_f64)
                    .build(),
                ConsumedCapacity::builder()
                    .capacity_units(1_f64)
                    .set_local_secondary_indexes(
                        HashMap::from_iter([(
                            "localIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(1_f64)
                                .read_capacity_units(1_f64)
                                .write_capacity_units(1_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .set_global_secondary_indexes(
                        HashMap::from_iter([(
                            "globalIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(1_f64)
                                .read_capacity_units(1_f64)
                                .write_capacity_units(1_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .read_capacity_units(1_f64)
                    .table(
                        Capacity::builder()
                            .capacity_units(1_f64)
                            .read_capacity_units(1_f64)
                            .write_capacity_units(1_f64)
                            .build(),
                    )
                    .table_name("Table")
                    .write_capacity_units(1_f64)
                    .build(),
            ),
            (
                ConsumedCapacity::builder()
                    .capacity_units(1_f64)
                    .set_local_secondary_indexes(
                        HashMap::from_iter([(
                            "localIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(1_f64)
                                .read_capacity_units(1_f64)
                                .write_capacity_units(1_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .set_global_secondary_indexes(
                        HashMap::from_iter([(
                            "globalIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(1_f64)
                                .read_capacity_units(1_f64)
                                .write_capacity_units(1_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .read_capacity_units(1_f64)
                    .table(
                        Capacity::builder()
                            .capacity_units(1_f64)
                            .read_capacity_units(1_f64)
                            .write_capacity_units(1_f64)
                            .build(),
                    )
                    .table_name("Table")
                    .write_capacity_units(1_f64)
                    .build(),
                ConsumedCapacity::builder()
                    .capacity_units(1_f64)
                    .set_local_secondary_indexes(
                        HashMap::from_iter([(
                            "localIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(1_f64)
                                .read_capacity_units(1_f64)
                                .write_capacity_units(1_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .set_global_secondary_indexes(
                        HashMap::from_iter([(
                            "globalIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(1_f64)
                                .read_capacity_units(1_f64)
                                .write_capacity_units(1_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .read_capacity_units(1_f64)
                    .table(
                        Capacity::builder()
                            .capacity_units(1_f64)
                            .read_capacity_units(1_f64)
                            .write_capacity_units(1_f64)
                            .build(),
                    )
                    .table_name("Table")
                    .write_capacity_units(1_f64)
                    .build(),
                ConsumedCapacity::builder()
                    .capacity_units(2_f64)
                    .set_local_secondary_indexes(
                        HashMap::from_iter([(
                            "localIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(2_f64)
                                .read_capacity_units(2_f64)
                                .write_capacity_units(2_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .set_global_secondary_indexes(
                        HashMap::from_iter([(
                            "globalIndex".to_owned(),
                            Capacity::builder()
                                .capacity_units(2_f64)
                                .read_capacity_units(2_f64)
                                .write_capacity_units(2_f64)
                                .build(),
                        )])
                        .into(),
                    )
                    .read_capacity_units(2_f64)
                    .table(
                        Capacity::builder()
                            .capacity_units(2_f64)
                            .read_capacity_units(2_f64)
                            .write_capacity_units(2_f64)
                            .build(),
                    )
                    .table_name("Table")
                    .write_capacity_units(2_f64)
                    .build(),
            ),
        ];

        for (a, b, c) in inputs {
            assert_eq!(extend_consumed_capacity(a, b), c);
        }
    }
}
