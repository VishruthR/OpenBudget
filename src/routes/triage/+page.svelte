<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "@iconify/svelte";
  import Button from "$lib/components/Button.svelte";
  import CategoryTable from "$lib/components/CategoryTable.svelte";
  import CategoryFormModal from "$lib/components/CategoryFormModal.svelte";
  import DeleteCategoryModal from "$lib/components/DeleteCategoryModal.svelte";
  import { categoriesApi } from "$lib/api/categories";
  import type { CategoryOverview, TransactionWithAccount } from "$lib/types";
    import FlashcardDeck from "$lib/components/FlashcardDeck.svelte";
    import { transactionsApi } from "$lib/api/transactions";

  let currentIndex = $state(0);
  
  let transactions = $state<TransactionWithAccount[]>([]);
  async function loadTransactions() {
    transactions = await transactionsApi.getTransactionsByCategory("Uncategorized");
  }
  onMount(loadTransactions);
  $inspect(transactions);

  let categories = $state<CategoryOverview[]>([]);
  async function loadCategories() {
    categories = await categoriesApi.getCategoryOverviews();
  }
  onMount(loadCategories);

  const handleTransactionAccept = () => {
    currentIndex++;
    console.log("accepted!");
  }
  const handleTransactionDiscard = () => {
    currentIndex++;
    console.log("discarded");
  }
  const handleReviewComplete = () => {
    console.log("review complete");
  }
</script>

<main class="page">
  <header class="page-header">
    <div class="page-heading">
      <h2 class="h2">Triage Transactions</h2>
      <p class="paragraph">
        See all your <strong>Uncategorized</strong> transactions in one place and quickly categorize them.
        Pennyful will remember your choices to save you time in the future.
      </p>
    </div>
  </header>
  <div class="body">
    <div class="flashcard-container">
      <FlashcardDeck
        bind:currentIndex
        transactions={transactions}
        discardText="Delete"
        acceptText="Submit"
        onDiscard={handleTransactionDiscard}
        onAccept={handleTransactionAccept}
        onComplete={handleReviewComplete}
      />
    </div>
  </div>
</main>

<style>
  .page {
    padding: 32px;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .page-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 24px;
  }

  .page-heading {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .body {
    width: 100%;
    display: flex;
    justify-content: center;
  }

  .flashcard-container {
    width: 100%;
    max-width: 500px;
  }
</style>
