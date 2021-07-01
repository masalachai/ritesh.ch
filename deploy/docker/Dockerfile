FROM rust:1.53.0-slim

ARG APP_ENV
ENV APP_ENV ${APP_ENV}
ENV APP_HOME /app

WORKDIR $APP_HOME

RUN mkdir $APP_HOME \
	&& apt update --fix-missing \
	&& apt install ca-certificates \
	&& curl https://github.com/mbrubeck/agate/releases/download/v3.1.0/agate.x86_64-unknown-linux-gnu.gz -o agate.gz \
	&& gunzip agate.gz \
	&& printf "agate --content /app/static/gemini --hostname ritesh.ch\nbin/ritesh-ch\n" > start.sh \
	&& rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

ADD . $APP_HOME

EXPOSE 8080

CMD ["start.sh"]
