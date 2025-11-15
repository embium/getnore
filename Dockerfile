# STAGE 1: Build the Rust Backend (No changes)
FROM rust:1.85-alpine as backend
WORKDIR /app
RUN apk add --no-cache libc-dev openssl-dev alpine-sdk
COPY ./packages/backend ./packages/backend
WORKDIR /app/packages/backend
RUN SQLX_OFFLINE=true RUSTFLAGS="-Ctarget-feature=-crt-static" cargo build --release --bin backend


# STAGE 2: Build the SvelteKit Frontend (No changes)
FROM node:20-alpine as frontend
WORKDIR /app
COPY ./packages/frontend/package.json ./packages/frontend/package-lock.json* ./
RUN npm install
COPY ./packages/frontend ./
RUN npm run build


# STAGE 3: Final Runtime Image (This is the new final stage)
FROM node:20-alpine as final
WORKDIR /app

# ⭐️ NEW: Install NGINX, supervisord, and curl (for healthcheck)
RUN apk add --no-cache nginx supervisor openssl

COPY --from=backend /app/packages/backend/etc/rbac_model.conf ./etc/rbac_model.conf
COPY --from=backend /app/packages/backend/target/release/backend /usr/local/bin/backend-server

# --- Setup SvelteKit Frontend ---
COPY --from=frontend /app/build ./frontend_server
COPY ./packages/frontend/package.json ./packages/frontend/package-lock.json* ./frontend_server/
WORKDIR /app/frontend_server
RUN npm install --omit=dev
WORKDIR /app

COPY nginx.conf /etc/nginx/nginx.conf
COPY supervisord.conf /etc/supervisord.conf

EXPOSE 8080

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
