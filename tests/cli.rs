use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::Path;

type TestFnResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_install() -> TestFnResult {
    // Arrange
    let ni_home_path = env::var("NI_HOME")?;
    assert_eq!(ni_home_path, "/tmp");

    // Act
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.args(["install", "https://github.com/e5pe0n/void.git"]);

    // Assert
    let output = cmd.assert().success();

    // Print the captured stdout to see the println! output
    println!(
        "Program stdout: {}",
        String::from_utf8_lossy(&output.get_output().stdout)
    );
    println!(
        "Program stderr: {}",
        String::from_utf8_lossy(&output.get_output().stderr)
    );

    let imported_dir_path = Path::new(&ni_home_path).join("void");
    assert!(imported_dir_path.join("tsconfig.json").exists());

    fs::remove_dir(imported_dir_path)?;
    Ok(())
}

#[test]
fn test_touch_file() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let ni_home_path = env::var("NI_HOME")?;
    assert_eq!(ni_home_path, "/tmp");

    // Act
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("foo.txt").write_stdin(
        r##"FROM node:20-slim AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

FROM base AS build
COPY . /usr/src/app
WORKDIR /usr/src/app
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run -r build
RUN pnpm deploy --filter=app1 --prod /prod/app1
RUN pnpm deploy --filter=app2 --prod /prod/app2

FROM base AS app1
COPY --from=build /prod/app1 /prod/app1
WORKDIR /prod/app1
EXPOSE 8000
CMD [ "pnpm", "start" ]

FROM base AS app2
COPY --from=build /prod/app2 /prod/app2
WORKDIR /prod/app2
EXPOSE 8001
CMD [ "pnpm", "start" ]
"##,
    );

    // Assert
    cmd.assert().success();
    let imported_path = Path::new(&ni_home_path).join("foo.txt");
    assert!(imported_path.exists());
    let content = fs::read_to_string(&imported_path)?;
    assert_eq!(
        content,
        r##"FROM node:20-slim AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

FROM base AS build
COPY . /usr/src/app
WORKDIR /usr/src/app
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run -r build
RUN pnpm deploy --filter=app1 --prod /prod/app1
RUN pnpm deploy --filter=app2 --prod /prod/app2

FROM base AS app1
COPY --from=build /prod/app1 /prod/app1
WORKDIR /prod/app1
EXPOSE 8000
CMD [ "pnpm", "start" ]

FROM base AS app2
COPY --from=build /prod/app2 /prod/app2
WORKDIR /prod/app2
EXPOSE 8001
CMD [ "pnpm", "start" ]
"##,
    );

    fs::remove_file(&imported_path)?;

    Ok(())
}
