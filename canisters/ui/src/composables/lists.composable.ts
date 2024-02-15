import { Ref, ref } from 'vue';
import { Pagination } from '~/types/app.types';

export const usePagination = ({ limit }: { limit?: number } = {}): Ref<Pagination> => {
  const pagination = ref<Pagination>({
    limit: limit ?? 25,
    totalPages: 1,
    selectedPage: 1,
  });

  return pagination;
};

export const useFetchList =
  <T>(
    fetchFn: (offset?: number, limit?: number) => Promise<T>,
    paginate?: { pagination: Ref<Pagination>; getTotal: (result: T) => number },
  ): (() => Promise<T>) =>
  async (): Promise<T> => {
    const result = paginate
      ? await fetchFn(
          (paginate.pagination.value.selectedPage - 1) * paginate.pagination.value.limit,
          paginate.pagination.value.limit,
        )
      : await fetchFn();
    if (paginate) {
      paginate.pagination.value.totalPages = Math.min(
        Math.ceil(paginate.getTotal(result) / paginate.pagination.value.limit),
        1,
      );
    }

    return result;
  };
