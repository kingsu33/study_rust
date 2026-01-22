# study_rust
cd C:\Users\User\개발\hello-rust
docker run --rm -it -v ${PWD}:/app -w /app rust:1.75 bash

# 도커 빌드 
docker build -f Dockerfile.dev -t rust-dev .

# docker run
docker run -it -v ${PWD}:/app rust-dev

# 처음 시작 시 
cargo init --bin

# 실행은 
cargo run

# 빌드는 
cargo build



## 레이어 책임

api/: HTTP 입구

요청(JSON) 파싱/검증 → service 호출 → 응답(JSON) 변환

✅ 프레임워크(axum/actix) 종속 OK

❌ 비즈니스 규칙/DB 쿼리 넣지 않기

service/: 비즈니스 규칙(유스케이스)

로그인/회원가입/권한/토큰 발급 같은 “규칙” 처리

✅ repository trait만 의존

❌ axum/actix 타입, HTTP StatusCode, Json 등 금지

domain/: 핵심 모델/에러/규칙

User, Email, UserId, PasswordHash, AppError 같은 “핵심”

✅ 순수 Rust 타입/검증

❌ DB/HTTP/Redis/외부 API 타입 금지

repository/: 저장소 인터페이스(=스프링 Repository 인터페이스 느낌)

trait UserRepository { ... }

✅ service가 의존하는 “약속”만 정의

❌ sqlx/orm 코드 금지(구현은 infra에서)

infra/: 구현체(어댑터)

DB(sqlx), Redis, 외부 API 등 “구체 구현”

✅ repository trait 구현

✅ 프레임워크는 모르도록 유지(가능하면)

교체 가능하게 만드는 3대 금지사항

service/domain/repository에서 axum/actix 타입 금지

service가 DB 구현(sqlx/orm)을 직접 만지지 않기

의존성 조립(wiring)은 app(main)에서만

“repo 구현체 → service 생성 → api에 주입” 이 흐름으로