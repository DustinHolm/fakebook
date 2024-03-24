FROM node:21-alpine AS dep_builder
WORKDIR /app
COPY ./package.json ./package-lock.json ./
RUN npm ci

FROM node:21-alpine AS builder
WORKDIR /app
COPY . ./
COPY --from=dep_builder /app ./
RUN mkdir __generated__
RUN npm run relay
RUN npm run build

FROM nginx:alpine AS runner
COPY --from=builder /app/dist /usr/share/nginx/html/
