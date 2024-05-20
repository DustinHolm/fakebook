FROM node:21-alpine
WORKDIR /app
COPY . ./
RUN npm ci
RUN npm run build
EXPOSE 3100
ENTRYPOINT ["npm", "run", "start"]