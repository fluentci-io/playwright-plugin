use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn install(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "playwright", "install", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn install_deps(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "playwright", "install-deps", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn screenshot(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "playwright", "screenshot", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn pdf(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "playwright", "pdf", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn test(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "playwright", "test", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn help(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "playwright", "help", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn version() -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "playwright", "--version"])?
        .stdout()?;
    Ok(stdout)
}
