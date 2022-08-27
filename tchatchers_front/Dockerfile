FROM node as asset_builder

WORKDIR /home/usr/front

COPY ./src ./src

COPY ./Cargo.toml ./Cargo.toml

COPY ./tailwind.config.js ./tailwind.config.js

COPY ./index.html ./index.html

COPY ./install_assets.sh ./install_assets.sh

RUN mkdir assets

RUN bash install_assets.sh

FROM rust:1.63 as src_builder

WORKDIR /usr/kikers/front

COPY --from=asset_builder /home/usr/front /usr/kikers/front

COPY ./favicon.ico ./favicon.ico

RUN rustup target add wasm32-unknown-unknown

RUN cargo install trunk

RUN trunk build --release

FROM nginx

COPY --from=src_builder /usr/kikers/front/dist /usr/share/nginx/html

COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
