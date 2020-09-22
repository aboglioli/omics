export interface INotificationBody {
  publication_id?: string;
  publication_name?: string;

  collection_id?: string;
  collection_name?: string;

  author_id?: string;
  author_username?: string;
  author_name?: string;
  author_lastname?: string;

  reader_id?: string;
  reader_username?: string;
  reader_name?: string;
  reader_lastname?: string;
}

/**
 * Notifications:
 * - welcome
 * - new-publication-from-followed-author
 * - publication-approved
 * - publication-rejected
 * - publication-liked
 * - author-followed
 */
export interface INotification {
  id: string;
  user_id: string;
  code: string;
  body: INotificationBody;
  read: boolean;
}
