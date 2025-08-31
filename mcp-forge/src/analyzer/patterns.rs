/// Trait for pattern detection, following SDK standards.
pub trait PatternDetect {
    fn detect_patterns(&self, code: &str, ext: &str) -> Vec<(String, f32)>;
}
    // Language-aware Builder detection
    fn detect_builder_lang(code: &str, ext: &str) -> f32 {
        match ext {
            "py" => {
                let has_chain = code.contains("return self") && code.contains("def build");
                if has_chain { 0.7 } else { 0.0 }
            }
            "js" | "ts" => {
                let has_chain = code.contains("return this") && code.contains("build(");
                if has_chain { 0.7 } else { 0.0 }
            }
            "java" => {
                let has_chain = code.contains("return this") && code.contains("build(");
                if has_chain { 0.7 } else { 0.0 }
            }
            _ => 0.0
        }
    }

    // Language-aware Repository detection
    fn detect_repository_lang(code: &str, ext: &str) -> f32 {
        match ext {
            "py" => {
                let has_repo = code.contains("def find_") && code.contains("def save_");
                if has_repo { 0.7 } else { 0.0 }
            }
            "js" | "ts" => {
                let has_repo = code.contains("find(") && code.contains("save(");
                if has_repo { 0.7 } else { 0.0 }
            }
            "java" => {
                let has_repo = code.contains("find") && code.contains("save");
                if has_repo { 0.7 } else { 0.0 }
            }
            _ => 0.0
        }
    }

    // Language-aware MVC detection
    fn detect_mvc_lang(code: &str, ext: &str) -> f32 {
        match ext {
            "py" | "js" | "ts" | "java" => {
                let has_model = code.contains("Model");
                let has_view = code.contains("View");
                let has_controller = code.contains("Controller");
                if has_model && has_view && has_controller { 0.8 } else { 0.0 }
            }
            _ => 0.0
        }
    }

    // Language-aware Dependency Injection detection
    fn detect_dependency_injection_lang(code: &str, ext: &str) -> f32 {
        match ext {
            "py" => {
                let has_di = code.contains("def __init__(self") && code.contains("service");
                if has_di { 0.7 } else { 0.0 }
            }
            "js" | "ts" => {
                let has_di = code.contains("constructor(") && code.contains("service");
                if has_di { 0.7 } else { 0.0 }
            }
            "java" => {
                let has_di = code.contains("@Inject") || code.contains("@Autowired");
                if has_di { 0.8 } else { 0.0 }
            }
            _ => 0.0
        }
    }

    // Language-aware Decorator detection
    fn detect_decorator_lang(code: &str, ext: &str) -> f32 {
        match ext {
            "py" => {
                let has_decorator = code.contains("@") && code.contains("def ");
                if has_decorator { 0.7 } else { 0.0 }
            }
            "js" | "ts" => {
                let has_decorator = code.contains("@") && code.contains("function ");
                if has_decorator { 0.7 } else { 0.0 }
            }
            "java" => {
                let has_decorator = code.contains("@Override") || code.contains("@Decorator");
                if has_decorator { 0.7 } else { 0.0 }
            }
            _ => 0.0
        }
    }
use std::collections::HashMap;

pub struct PatternDetector {
    patterns: HashMap<String, fn(&str) -> f32>,
}

impl PatternDetect for PatternDetector {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        patterns.insert("Singleton".to_string(), Self::detect_singleton);
        patterns.insert("Factory".to_string(), Self::detect_factory);
        patterns.insert("Observer".to_string(), Self::detect_observer);
        patterns.insert("Builder".to_string(), Self::detect_builder);
        patterns.insert("Repository".to_string(), Self::detect_repository);
        patterns.insert("MVC".to_string(), Self::detect_mvc);
        patterns.insert("Dependency Injection".to_string(), Self::detect_dependency_injection);
        patterns.insert("Decorator".to_string(), Self::detect_decorator);
        Self { patterns }
    }

    /// Detect patterns, language-aware
    pub fn detect_patterns(&self, code: &str, ext: &str) -> HashMap<String, f32> {
        let mut results = HashMap::new();
        for (name, detector) in &self.patterns {
            let confidence = match name.as_str() {
                "Singleton" => Self::detect_singleton_lang(code, ext),
                "Factory" => Self::detect_factory_lang(code, ext),
                "Observer" => Self::detect_observer_lang(code, ext),
                "Builder" => Self::detect_builder_lang(code, ext),
                "Repository" => Self::detect_repository_lang(code, ext),
                "MVC" => Self::detect_mvc_lang(code, ext),
                "Dependency Injection" => Self::detect_dependency_injection_lang(code, ext),
                "Decorator" => Self::detect_decorator_lang(code, ext),
                _ => detector(code),
            };
            if confidence > 0.0 {
                results.insert(name.clone(), confidence);
            }
        }
        results
    }

    // Language-aware Singleton detection
    fn detect_singleton_lang(code: &str, ext: &str) -> f32 {
        match ext {
            "py" => Self::detect_singleton(code),
            "js" | "ts" => {
                let has_instance = code.contains("static instance") || code.contains("let instance") || code.contains("var instance");
                let has_get_instance = code.contains("getInstance(") || code.contains("get_instance(");
                if has_instance && has_get_instance { 0.8 } else if has_get_instance { 0.5 } else { 0.0 }
            }
            "java" => {
                let has_private_ctor = code.contains("private") && code.contains("static") && code.contains("getInstance(");
                if has_private_ctor { 0.9 } else { 0.0 }
            }
            _ => 0.0
        }
    }

    // Language-aware Factory detection
    fn detect_factory_lang(code: &str, ext: &str) -> f32 {
        match ext {
            "py" => {
                let has_factory = code.contains("def create_") && code.contains("return");
                if has_factory { 0.7 } else { 0.0 }
            }
            "js" | "ts" => {
                let has_factory = code.contains("function create") && code.contains("return");
                if has_factory { 0.7 } else { 0.0 }
            }
            "java" => {
                let has_factory = code.contains("public") && code.contains("create") && code.contains("return");
                if has_factory { 0.7 } else { 0.0 }
            }
            _ => 0.0
        }
    }

    // Language-aware Observer detection
    fn detect_observer_lang(code: &str, ext: &str) -> f32 {
        match ext {
            "py" => {
                let has_subscribe = code.contains("def subscribe") || code.contains("def attach");
                let has_notify = code.contains("def notify") || code.contains("def update");
                if has_subscribe && has_notify { 0.8 } else { 0.0 }
            }
            "js" | "ts" => {
                let has_subscribe = code.contains("subscribe(") || code.contains("on(");
                let has_notify = code.contains("notify(") || code.contains("emit(");
                if has_subscribe && has_notify { 0.8 } else { 0.0 }
            }
            "java" => {
                let has_add_observer = code.contains("addObserver(") || code.contains("addListener(");
                let has_notify = code.contains("notifyObservers(") || code.contains("update(");
                if has_add_observer && has_notify { 0.8 } else { 0.0 }
            }
            _ => 0.0
        }
    }

    fn detect_singleton(code: &str) -> f32 {
        // Basic Singleton detection for Python:
        // Look for class with private constructor and getInstance/static method
        let has_private_ctor = code.contains("def __init__(self)") && code.contains("raise Exception") && code.contains("_instance = None");
        let has_get_instance = code.contains("@staticmethod") && code.contains("def get_instance(");
        if has_private_ctor && has_get_instance {
            0.9
        } else if has_get_instance {
            0.5
        } else {
            0.0
        }
    }

    fn detect_factory(code: &str) -> f32 {
        // Implement detection logic for Factory pattern
        0.0 // Placeholder confidence score
    }

    fn detect_observer(code: &str) -> f32 {
        // Implement detection logic for Observer pattern
        0.0 // Placeholder confidence score
    }

    fn detect_builder(code: &str) -> f32 {
        // Implement detection logic for Builder pattern
        0.0 // Placeholder confidence score
    }

    fn detect_repository(code: &str) -> f32 {
        // Implement detection logic for Repository pattern
        0.0 // Placeholder confidence score
    }

    fn detect_mvc(code: &str) -> f32 {
        // Implement detection logic for MVC pattern
        0.0 // Placeholder confidence score
    }

    fn detect_dependency_injection(code: &str) -> f32 {
        // Implement detection logic for Dependency Injection pattern
        0.0 // Placeholder confidence score
    }

    fn detect_decorator(code: &str) -> f32 {
        // Implement detection logic for Decorator pattern
        0.0 // Placeholder confidence score
    }
}