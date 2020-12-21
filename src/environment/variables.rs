use std::env;

const UNSET_ENVIRONMENT_VARIABLE_ERROR : &str = "environment variable must be set";

fn format_env_variable_error(name: &str) -> String {
    format!("{} {}", name, UNSET_ENVIRONMENT_VARIABLE_ERROR)
}

pub fn expect_variable(name: &str) -> String {
    return env::var(&name)
            .expect(&format_env_variable_error(name))
}