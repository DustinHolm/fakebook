FROM node:21-alpine AS builder
WORKDIR /app
COPY . ./
RUN npm ci
RUN mkdir __generated__
RUN npm run relay
RUN npm run build

FROM nginx:alpine AS runner
COPY --from=builder /app/dist /usr/share/nginx/html/
COPY ./nginx.conf /etc/nginx/conf.d/default.conf
