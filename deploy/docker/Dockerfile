FROM rust:1.53.0-slim

ARG APP_ENV
ENV APP_ENV ${APP_ENV}
ENV APP_HOME /app

WORKDIR $APP_HOME

RUN apt update --fix-missing \
	&& apt install -y ca-certificates wget \
	&& wget https://github.com/mbrubeck/agate/releases/download/v3.1.0/agate.x86_64-unknown-linux-gnu.gz -O agate.gz \
	&& gunzip agate.gz \
	&& chmod +x agate \
	&& printf "./agate --content /app/static/gemini --hostname ritesh.ch &\nbin/ritesh-ch\n" > start.sh \
	&& rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

ADD . $APP_HOME

EXPOSE 1965
EXPOSE 8080

CMD ["sh", "start.sh"]
