use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::Path;

type TestFnResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_write_file() -> TestFnResult {
    // Arrange
    let temp = assert_fs::TempDir::new().unwrap();
    unsafe {
        env::set_var("NI_HOME", temp.to_str().unwrap());
    }
    let ni_home_path = env::var("NI_HOME")?;
    assert_eq!(ni_home_path, temp.to_str().unwrap());

    // Act
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("foo.txt").write_stdin(
        r##"FROM node:20-slim AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY . /app
WORKDIR /app

FROM base AS prod-deps
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --prod --frozen-lockfile

FROM base AS build
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run build

FROM base
COPY --from=prod-deps /app/node_modules /app/node_modules
COPY --from=build /app/dist /app/dist
EXPOSE 8000
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
COPY . /app
WORKDIR /app

FROM base AS prod-deps
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --prod --frozen-lockfile

FROM base AS build
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run build

FROM base
COPY --from=prod-deps /app/node_modules /app/node_modules
COPY --from=build /app/dist /app/dist
EXPOSE 8000
CMD [ "pnpm", "start" ]
"##,
    );

    temp.close().unwrap();

    Ok(())
}
