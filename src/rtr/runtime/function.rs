use std::collections::HashMap;
use crate::rtr::log::{RTRLog, RTRLogKind};
use crate::rtr::runtime::memory::{MemPointer, Memory};
use crate::{print_raw, print_warn, print_error, Log, LogKind, LogSource, print_log};
use crate::rtr::error::Error;
use crate::rtr::runtime::values::arr_value::RTRArr;
use crate::rtr::runtime::values::bool_value::RTRBool;
use crate::rtr::runtime::values::null_value::RTRNull;
use crate::rtr::runtime::values::num_value::RTRNum;
use crate::rtr::runtime::values::obj_value::RTRObj;
use crate::rtr::runtime::values::str_value::RTRStr;
use crate::rtr::runtime::values::type_value::RTRType;
use crate::shared::utils::{chr, ord};

#[derive(Debug, Clone)]
pub enum BuiltinFunction {
    Log,
    Error,
    Return,
    Typeof,
    Length,
    
    // mathematical
    Min,
    Max,
    
    Abs,
    Sqrt,
    
    Round,
    Floor,
    Ceil,
    
    // string
    Join,
    Split,
    
    Chr,
    Ord,
    
    ToUpper,
    ToLower,
    ToTitle,
    
    // array
    Item,
    Range,
    
    // object
    Keys,
    Values,
    Has,
    Obj,
    
    // logical
    All,
    Any,
    Not
}

impl BuiltinFunction {
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unnecessary_wraps)]
    pub fn call(&self, logs: &mut Vec<RTRLog>, memory: &mut Memory, args: &[MemPointer]) -> Result<MemPointer, Error> {
        match self {
            BuiltinFunction::Log => {
                let text = args
                    .iter()
                    .map(|ptr| memory.get(*ptr))
                    .map(|v| v.stringify(memory))
                    .collect::<Vec<_>>()
                    .join(" ");
                
                print_log!(LogSource::Rtr, "{text}");
                
                logs.push(RTRLog {
                    kind: RTRLogKind::Log,
                    text
                });
            },
            BuiltinFunction::Error => {
                // TODO: have this error and then return and call the event
                let text = args
                    .iter()
                    .map(|ptr| memory.get(*ptr))
                    .map(|v| v.stringify(memory))
                    .collect::<Vec<_>>()
                    .join(" ");
                
                print_error!(LogSource::Rtr, "{text}");
                
                logs.push(RTRLog {
                    kind: RTRLogKind::Error,
                    text
                });
            },
            BuiltinFunction::Return => {
                // return is handled in call instruction
            }
            BuiltinFunction::Typeof => {
                return Ok(
                    memory.alloc(Box::new(RTRType {
                        data: memory.get(args[0]).get_type()
                    }))
                )
            }
            BuiltinFunction::Length => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Box::new(RTRNum {
                        data: memory.get(args[0]).length(memory) as f32
                    }))
                );
            }
            
            // mathematical
            BuiltinFunction::Min => {
                // TODO: check amount of args
                let mut iter = args.iter();
                let mut val = memory.get(*iter.next().unwrap()).numbify();
                
                for arg in iter {
                    let arg = memory.get(*arg).numbify();
                    val = val.min(arg);
                }
                
                return Ok(
                    memory.alloc(Box::new(RTRNum {
                        data: val
                    }))
                );
            }
            BuiltinFunction::Max => {
                // TODO: check amount of args
                let mut iter = args.iter();
                let mut val = memory.get(*iter.next().unwrap()).numbify();
                
                for arg in iter {
                    let arg = memory.get(*arg).numbify();
                    val = val.max(arg);
                }
                
                return Ok(
                    memory.alloc(Box::new(RTRNum {
                        data: val
                    }))
                );
            }
            
            BuiltinFunction::Abs => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Box::new(RTRNum {
                        data: memory.get(args[0]).numbify().abs()
                    }))
                );
            }
            BuiltinFunction::Sqrt => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Box::new(RTRNum {
                        data: memory.get(args[0]).numbify().sqrt()
                    }))
                );
            }
            
            // Round
            BuiltinFunction::Round => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Box::new(RTRNum {
                        data: memory.get(args[0]).numbify().round()
                    }))
                );
            }
            BuiltinFunction::Floor => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Box::new(RTRNum {
                        data: memory.get(args[0]).numbify().floor()
                    }))
                );
            }
            BuiltinFunction::Ceil => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Box::new(RTRNum {
                        data: memory.get(args[0]).numbify().ceil()
                    }))
                );
            }
            
            // string
            BuiltinFunction::Join => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Box::new(RTRStr {
                        data: args
                            .iter()
                            .map(|ptr| memory.get(*ptr).stringify(memory))
                            .reduce(|a, b| {
                                format!("{a}{b}")
                            }).unwrap_or(String::new())
                    }))
                );
            }
            BuiltinFunction::Split => {
                // TODO: check amount of args
                let str = memory.get(args[0]).stringify(memory);
                let sep = memory.get(args[1]).stringify(memory);
                
                let val = RTRArr {
                    items: str.split(sep.as_str())
                        .map(|s| memory.alloc(
                            Box::new(RTRStr { data: s.to_string() })
                        ))
                        .collect()
                };
                
                return Ok(
                    memory.alloc(Box::new(val))
                );
            }
            BuiltinFunction::Chr => {
                // TODO: check amount of args
                let num = memory.get(args[0]).numbify();
                return Ok(
                    memory.alloc(
                        Box::new(RTRStr {
                            data: chr(num)
                        })
                    )
                )
            }
            BuiltinFunction::Ord => {
                // TODO: check amount of args
                let str = memory.get(args[0]).stringify(memory);
                return Ok(
                    memory.alloc(
                        Box::new(RTRNum {
                            data: ord(&str)
                        })
                    )
                )
            }
            
            BuiltinFunction::ToUpper => {
                // TODO: handle incorrect amount of args
                let str = memory.get(args[0]).stringify(memory);
                return Ok(
                    memory.alloc(
                        Box::new(RTRStr {
                            data: str.to_uppercase()
                        })
                    )
                )
            }
            BuiltinFunction::ToLower => {
                // TODO: handle incorrect amount of args
                let str = memory.get(args[0]).stringify(memory);
                return Ok(
                    memory.alloc(
                        Box::new(RTRStr {
                            data: str.to_lowercase()
                        })
                    )
                )
            }
            BuiltinFunction::ToTitle => {
                // TODO: handle incorrect amount of args
                let str = memory.get(args[0]).stringify(memory);
                let words: Vec<String> = str.split(' ')
                    .map(|w| w[0..1].to_uppercase() + &w[1..].to_lowercase())
                    .collect();
                return Ok(
                    memory.alloc(
                        Box::new(RTRStr {
                            data: words.join(" ")
                        })
                    )
                )
            }
            
            // array
            BuiltinFunction::Item => {
                // TODO: handle incorrect amount of args
                let value = memory.get(args[0]).clone_box();
                let idx = memory.get(args[1]).clone_box();
                return Ok(
                    value.get_item(memory, &*idx)
                );
            }
            BuiltinFunction::Range => {
                let mut items = Vec::new();
                
                // TODO: handle incorrect amount of args
                let a = memory.get(args[0]).numbify().trunc() as usize;
                let b = memory.get(args[1]).numbify().trunc() as usize;
                
                for i in a..=b {
                    items.push(memory.alloc(
                        Box::new(RTRNum { data: i as f32 })
                    ));
                }
                
                return Ok(memory.alloc(
                    Box::new(RTRArr { items })
                ))
            }
            
            // object
            BuiltinFunction::Keys => {
                // TODO: handle incorrect amount of args
                let keys = memory.get(args[0])
                    .clone_box()
                    .keys(memory);
                return Ok(memory.alloc(Box::new(RTRArr {
                    items: keys
                })));
            }
            BuiltinFunction::Values => {
                // TODO: handle incorrect amount of args
                let keys = memory.get(args[0])
                    .clone_box()
                    .values(memory);
                return Ok(memory.alloc(Box::new(RTRArr {
                    items: keys
                })));
            }
            BuiltinFunction::Has => {
                // TODO: handle incorrect amount of args
                let value = memory.get(args[0]).clone_box();
                let key = memory.get(args[1]).clone_box();
                let data = value.has(memory, &*key);
                return Ok(memory.alloc(Box::new(RTRBool {
                    data
                })));
            }
            BuiltinFunction::Obj => {
                return Ok(memory.alloc(Box::new(RTRObj {
                    data: HashMap::new()
                })));
            }
            
            // logical
            BuiltinFunction::All => {
                return Ok(memory.alloc(Box::new(RTRBool {
                    data: args
                        .iter()
                        .all(|v| memory.get(*v).boolify())
                })))
            }
            BuiltinFunction::Any => {
                return Ok(memory.alloc(Box::new(RTRBool {
                    data: args
                        .iter()
                        .any(|v| memory.get(*v).boolify())
                })))
            }
            BuiltinFunction::Not => {
                // TODO: handle incorrect amount of args
                return Ok(memory.alloc(Box::new(RTRBool {
                    data: !memory.get(args[0]).boolify()
                })))
            }
        }
        
        Ok(memory.alloc(Box::new(RTRNull)))
    }
}
