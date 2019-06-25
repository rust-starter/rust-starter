FROM circleci/rust:1.35

COPY --chown=circleci:circleci . /home/circleci/project

WORKDIR /home/circleci/project

RUN cargo fetch
