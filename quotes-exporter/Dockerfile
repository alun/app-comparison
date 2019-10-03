FROM mhart/alpine-node:10

RUN mkdir /app
WORKDIR /app

COPY package.json /app
COPY src /app/src
COPY index.js /app

RUN npm i
CMD node index
