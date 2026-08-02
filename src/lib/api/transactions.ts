import type { PaginedSortedTransactionsResponse, TransactionWithAccount } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

const syncTransactions = async (): Promise<void> => {
    return (await invoke("sync_transactions")) ;
}

const getPaginatedSortedTransactions = async (
  page: number,
  page_size: number,
  sort_col: string | null,
  sort_dir: string | null
): Promise<PaginedSortedTransactionsResponse> => {
  return (await invoke("get_paginated_sorted_transactions", {
    page: page,
    pageSize: page_size,
    sortCol: sort_col,
    sortDir: sort_dir
  })) as PaginedSortedTransactionsResponse;
}

const getTransactionsByCategory = async (categoryName: string): Promise<TransactionWithAccount[]> => {
  return (await invoke("get_transactions_by_category", { categoryName })) as TransactionWithAccount[];
}

const getNumTransactionsByCategory = async (categoryName: string): Promise<number> => {
  return (await invoke("get_num_transactions_by_category", { categoryName })) as number;
}

const updateTransactionCategory = async (transactionId: number, categoryId: number): Promise<void> => {
  await invoke("update_transaction_category", { transactionId, categoryId });
}

export const transactionsApi = {
  syncTransactions,
  getPaginatedSortedTransactions,
  updateTransactionCategory,
  getTransactionsByCategory,
  getNumTransactionsByCategory
}
