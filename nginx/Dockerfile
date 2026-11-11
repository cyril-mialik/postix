FROM nginx:alpine

COPY --from=client_builder /dist /usr/share/nginx/html

COPY nginx/nginx.conf /etc/nginx/nginx.conf
COPY nginx/conf.d /etc/nginx/conf.d

EXPOSE 80
