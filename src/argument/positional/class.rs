use crate::argument_class::ArgumentClass;

pub enum Class {
    String(usize, usize),
    Integer(usize, usize),
    Float(usize, usize),
}

impl Class {
    pub fn new<S: AsRef<str>>(class: S) -> Self {
        match class.as_ref() {
            "string" => Class::String(0, 0),
            "integer" => Class::Integer(0, 0),
            "float" => Class::Float(0, 0),
            _ => panic!("Invalid class {}", class.as_ref()),
        }
    }

    pub fn maximum(&mut self, maximum: usize) {
        match self {
            Class::String(min, max) | Class::Integer(min, max) | Class::Float(min, max) => {
                if maximum != 0 && *min != 0 && *min > maximum {
                    panic!("Attempting to set maximum count to lower than minimum count");
                }

                *max = maximum
            }
        }
    }

    pub fn minimum(&mut self, minimum: usize) {
        match self {
            Class::String(min, max) | Class::Integer(min, max) | Class::Float(min, max) => {
                if *max != 0 && minimum != 0 && *max < minimum {
                    panic!("Attempting to set minimum count to higher than maximum count");
                }

                *min = minimum
            }
        }
    }

    pub fn range(&mut self, minimum: usize, maximum: usize) {
        match self {
            Class::String(min, max) | Class::Integer(min, max) | Class::Float(min, max) => {
                if minimum != 0 && maximum != 0 && minimum > maximum {
                    panic!("Attempting to set maximum count to lower than minimum count");
                }

                *max = maximum;
                *min = minimum;
            }
        }
    }

    pub fn required(&mut self, required: bool) {
        match self {
            Class::String(min, _) | Class::Integer(min, _) | Class::Float(min, _) => {
                if required {
                    if *min == 0 {
                        *min = 1;
                    }
                } else {
                    *min = 0;
                }
            }
        }
    }

    pub fn generate_instance(&self) -> ArgumentClass {
        match self {
            Class::String(_, _) => ArgumentClass::String(Vec::new()),
            Class::Integer(_, _) => ArgumentClass::Integer(Vec::new()),
            Class::Float(_, _) => ArgumentClass::Float(Vec::new()),
        }
    }

    pub fn get_maximum(&self) -> usize {
        match self {
            Class::String(_, max) | Class::Integer(_, max) | Class::Float(_, max) => *max,
        }
    }

    pub fn get_minimum(&self) -> usize {
        match self {
            Class::String(min, _) | Class::Integer(min, _) | Class::Float(min, _) => *min,
        }
    }
}
