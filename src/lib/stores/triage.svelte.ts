import { transactionsApi } from "$lib/api/transactions";

/**
 * Sidebar displays this number
 * Category updates should call loadUncategorizedCount after updating
 * a transactions category
 */
export const triageStore = $state({ uncategorizedCount: 0 });

export async function loadUncategorizedCount(): Promise<void> {
  triageStore.uncategorizedCount =
    await transactionsApi.getNumTransactionsByCategory("Uncategorized");
}
