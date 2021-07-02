FROM rust:1.53.0-slim

ARG APP_ENV
ENV APP_ENV ${APP_ENV}
ENV APP_HOME /app

WORKDIR $APP_HOME
EXPOSE 8080 1965

RUN apt update --fix-missing \
	&& apt install -y ca-certificates \
	&& printf "bin/ritesh-ch-gemini >> /dev/stdout 2>&1 &\nbin/ritesh-ch-http\n" > start.sh \
	&& rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

ADD . $APP_HOME

CMD ["sh", "start.sh"]
