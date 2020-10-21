export interface IPagination<T> {
  offset: number;
  limit: number;
  total: number;
  matching_criteria: number;
  count: number;
  items: T[];
}
