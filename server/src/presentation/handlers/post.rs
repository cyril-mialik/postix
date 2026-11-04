use actix_web::{HttpResponse, delete, get, patch, post, put, web};

use crate::{
    application::post::PostService,
    domain::entities::{errors::Error, post::PostError},
    presentation::dto::post::{
        DeletePostRequest, PostRequest, PostResponse, PostsResponse, QueryPostId, QueryPostUserId,
        UpdatePostContentRequest, UpdatePostRequest, UpdatePostTitleRequest,
    },
};

#[get("/post")]
pub async fn get_post_by_id(
    service: web::Data<PostService>,
    query: web::Query<QueryPostId>,
) -> Result<HttpResponse, Error> {
    let post = service
        .find_by_id(query.into_inner().into())
        .await?
        .ok_or(Error::Post(PostError::NotFound))?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

#[get("/posts")]
pub async fn get_post_by_user_id(
    service: web::Data<PostService>,
    query: web::Query<QueryPostUserId>,
) -> Result<HttpResponse, Error> {
    let posts = service.find_by_user_id(query.into_inner().into()).await?;

    Ok(HttpResponse::Ok().json(PostsResponse::from(posts)))
}

#[post("/post")]
pub async fn create_post(
    service: web::Data<PostService>,
    payload: web::Json<PostRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.create_post(payload.into_inner().into()).await?;

    Ok(HttpResponse::Created().json(PostResponse::from(post)))
}

#[put("/post")]
pub async fn update_post(
    service: web::Data<PostService>,
    payload: web::Json<UpdatePostRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.update_post(payload.into_inner().into()).await?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

#[patch("/post/title")]
pub async fn update_title(
    service: web::Data<PostService>,
    payload: web::Json<UpdatePostTitleRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.update_title(payload.into_inner().into()).await?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

#[patch("/post/content")]
pub async fn update_content(
    service: web::Data<PostService>,
    payload: web::Json<UpdatePostContentRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.update_content(payload.into_inner().into()).await?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

#[delete("/post")]
pub async fn delete_post(
    service: web::Data<PostService>,
    payload: web::Json<DeletePostRequest>,
) -> Result<HttpResponse, Error> {
    service.delete_post(payload.into_inner().into()).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::post::PostService;
    use crate::domain::entities::post::Post;
    use crate::domain::repositories::post::PostRepository;
    use actix_web::{App, test, web};
    use chrono::Utc;
    use mockall::mock;
    use mockall::predicate::*;
    use std::sync::Arc;
    use uuid::Uuid;

    mock! {
        pub PostRepo {}
        #[async_trait::async_trait]
        impl PostRepository for PostRepo {
            async fn create_post(&self, dto: crate::domain::entities::post::CreatePostDto) -> Result<Post, Error>;
            async fn find_by_id(&self, dto: crate::domain::entities::post::FindPostByIdDto) -> Result<Option<Post>, Error>;
            async fn find_by_user_id(&self, dto: crate::domain::entities::post::FindPostByUserIdDto) -> Result<Vec<Post>, Error>;
            async fn update_post(&self, dto: crate::domain::entities::post::UpdatePostDto) -> Result<Post, Error>;
            async fn update_title(&self, dto: crate::domain::entities::post::UpdatePostTitleDto) -> Result<Post, Error>;
            async fn update_content(&self, dto: crate::domain::entities::post::UpdatePostContentDto) -> Result<Post, Error>;
            async fn delete_post(&self, dto: crate::domain::entities::post::DeletePostDto) -> Result<(), Error>;
        }
    }

    fn mock_post() -> Post {
        Post {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[actix_web::test]
    async fn test_get_post_by_id_success() {
        let mut mock_repo = MockPostRepo::new();
        let post = mock_post();
        let post_id = post.id;
        let post_clone = post.clone();

        mock_repo
            .expect_find_by_id()
            .withf(move |dto| dto.id == post_id)
            .once()
            .returning(move |_| Ok(Some(post_clone.clone())));

        let service = PostService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(get_post_by_id),
        )
        .await;

        let req = test::TestRequest::get()
            .uri(&format!("/post?id={}", post_id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
        let body: PostResponse = test::read_body_json(resp).await;
        assert_eq!(body.id, post_id);
    }

    #[actix_web::test]
    async fn test_get_post_by_id_not_found() {
        let mut mock_repo = MockPostRepo::new();
        let post_id = Uuid::new_v4();

        mock_repo.expect_find_by_id().once().returning(|_| Ok(None));

        let service = PostService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(get_post_by_id),
        )
        .await;

        let req = test::TestRequest::get()
            .uri(&format!("/post?id={}", post_id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error() || resp.status().is_server_error());
    }

    #[actix_web::test]
    async fn test_get_post_by_user_id_success() {
        let mut mock_repo = MockPostRepo::new();
        let user_id = Uuid::new_v4();
        let posts = vec![mock_post(), mock_post()];
        let posts_clone = posts.clone();

        mock_repo
            .expect_find_by_user_id()
            .withf(move |dto| dto.user_id == user_id)
            .once()
            .returning(move |_| Ok(posts_clone.clone()));

        let service = PostService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(get_post_by_user_id),
        )
        .await;

        let req = test::TestRequest::get()
            .uri(&format!("/posts?user_id={}", user_id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
        let body: PostsResponse = test::read_body_json(resp).await;
        assert_eq!(body.posts.len(), 2);
    }

    #[actix_web::test]
    async fn test_create_post_success() {
        let mut mock_repo = MockPostRepo::new();
        let post = mock_post();
        let post_id = post.id;
        let post_clone = post.clone();
        let user_id = post.user_id;

        mock_repo.expect_find_by_id().once().returning(|_| Ok(None));

        mock_repo
            .expect_create_post()
            .once()
            .returning(move |_| Ok(post_clone.clone()));

        let service = PostService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(create_post),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/post")
            .set_json(PostRequest {
                user_id,
                title: "Test Title".to_string(),
                content: "Test Content".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 201);
        let body: PostResponse = test::read_body_json(resp).await;
        assert_eq!(body.id, post_id);
    }

    #[actix_web::test]
    async fn test_update_post_success() {
        let mut mock_repo = MockPostRepo::new();
        let post = mock_post();
        let post_id = post.id;
        let post_clone = post.clone();

        mock_repo
            .expect_update_post()
            .withf(move |dto| dto.id == post_id)
            .once()
            .returning(move |_| Ok(post_clone.clone()));

        let service = PostService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(update_post),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/post")
            .set_json(UpdatePostRequest {
                id: post_id,
                new_title: "New Title".to_string(),
                new_content: "New Content".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_update_title_success() {
        let mut mock_repo = MockPostRepo::new();
        let post = mock_post();
        let post_id = post.id;
        let post_for_find = post.clone();
        let post_for_update = post.clone();

        mock_repo
            .expect_find_by_id()
            .once()
            .returning(move |_| Ok(Some(post_for_find.clone())));

        mock_repo
            .expect_update_title()
            .withf(move |dto| dto.id == post_id && dto.new_title == "New Title")
            .once()
            .returning(move |_| Ok(post_for_update.clone()));

        let service = PostService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(update_title),
        )
        .await;

        let req = test::TestRequest::patch()
            .uri("/post/title")
            .set_json(UpdatePostTitleRequest {
                id: post_id,
                new_title: "  New Title  ".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_update_content_success() {
        let mut mock_repo = MockPostRepo::new();
        let post = mock_post();
        let post_id = post.id;
        let post_for_find = post.clone();
        let post_for_update = post.clone();

        mock_repo
            .expect_find_by_id()
            .once()
            .returning(move |_| Ok(Some(post_for_find.clone())));

        mock_repo
            .expect_update_content()
            .withf(move |dto| dto.id == post_id && dto.new_content == "New Content")
            .once()
            .returning(move |_| Ok(post_for_update.clone()));

        let service = PostService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(update_content),
        )
        .await;

        let req = test::TestRequest::patch()
            .uri("/post/content")
            .set_json(UpdatePostContentRequest {
                id: post_id,
                new_content: "  New Content  ".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_delete_post_success() {
        let mut mock_repo = MockPostRepo::new();
        let post_id = Uuid::new_v4();

        mock_repo
            .expect_delete_post()
            .withf(move |dto| dto.id == post_id)
            .once()
            .returning(|_| Ok(()));

        let service = PostService::new(Arc::new(mock_repo));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .service(delete_post),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/post")
            .set_json(DeletePostRequest { id: post_id })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 204);
    }
}
