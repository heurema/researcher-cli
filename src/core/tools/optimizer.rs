use crate::core::tools::code_runner::CodeRunner;
use anyhow::Result;
use futures::future::join_all;

pub struct Optimizer;

impl Optimizer {
    /// Optimize a python function over a list of parameter values.
    /// Expects `code` to be a python template with `{}` for the parameter.
    pub async fn optimize(code: &str, params: Vec<String>) -> Result<(String, String)> {
        let mut tasks = Vec::new();

        for p in params {
            let task_code = code.replace("{}", &p);
            tasks.push(tokio::spawn(async move {
                (p.clone(), CodeRunner::run_python(&task_code).await)
            }));
        }

        let results = join_all(tasks).await;
        
        let mut best_val: Option<f64> = None;
        let mut best_param = String::new();
        let mut best_output = String::new();

        for res in results {
            if let Ok((param, Ok(output))) = res {
                if let Ok(val) = output.trim().parse::<f64>() {
                    if best_val.is_none() || val > best_val.unwrap() {
                        best_val = Some(val);
                        best_param = param;
                        best_output = output;
                    }
                }
            }
        }

        if best_val.is_none() {
            anyhow::bail!("Optimization failed: No numeric results found.");
        }

        Ok((best_param, best_output))
    }
}
