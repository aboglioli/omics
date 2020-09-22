#[derive(Debug, Clone)]
pub enum Body {
    FollowedYou {
        reader_name: String,
    },
    ReviewedYourPublication {
        publication_id: String,
        publication_name: String,
        reader_name: String,
        comment: String,
    },
    LikedYourPublication {
        reader_name: String,
    },
    NewPublicationFromFollowedAuthor {
        author_id: String,
        author_name: String,
        publication_id: String,
        publication_name: String,
    },
    PublicationFromFollowedAuthorUpdated {
        author_id: String,
        author_name: String,
        publication_id: String,
        publication_name: String,
    },
    CollectionFromFollowedAuthor,
}
