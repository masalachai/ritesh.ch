FROM rust:1.53.0-slim

ARG APP_ENV
ENV APP_ENV ${APP_ENV}
ENV APP_HOME /app

RUN mkdir $APP_HOME \
	&& apt update --fix-missing \
	&& apt install ca-certificates \
	&& rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

WORKDIR $APP_HOME

ADD . $APP_HOME

RUN mv deploy/static ./ && mv deploy/templates ./

EXPOSE 8080

CMD ["bin/ritesh-ch"]
