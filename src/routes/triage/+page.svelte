<script lang="ts">
  import { onMount } from "svelte";
  import { categoriesApi } from "$lib/api/categories";
  import type { CategoryOverview, TransactionWithAccount } from "$lib/types";
  import FlashcardDeck from "$lib/components/FlashcardDeck.svelte";
  import { transactionsApi } from "$lib/api/transactions";
  import UncategorizedFlashcard from "$lib/components/UncategorizedFlashcard.svelte";
  import Button from "$lib/components/Button.svelte";
  import { setUncategorizedCount, loadUncategorizedCount } from "$lib/stores/triage.svelte";

  let index = $state(0);
  let isAnimating = $state(false);
  let deck = $state<ReturnType<typeof FlashcardDeck>>();
  let isReviewComplete = $state(false);
  
  let transactions = $state<TransactionWithAccount[]>([]);
  async function loadTransactions() {
    transactions = await transactionsApi.getTransactionsByCategory("Uncategorized");
    setUncategorizedCount(transactions.length);
  }
  onMount(loadTransactions);
  $inspect(transactions);

  let categories = $state<CategoryOverview[]>([]);
  let loadingCategories = $state<boolean>(false);
  async function loadCategories() {
    loadingCategories = true;
    categories = await categoriesApi.getCategoryOverviews();
    loadingCategories = false;
  }
  onMount(loadCategories);

  const handleCategoryUpdate = (transactionId: number, categoryId: number) => {
    transactionsApi.updateTransactionCategory(transactionId, categoryId)
      .then(() => loadUncategorizedCount());
    handleNext();
  }

  const handleNext = () => {
    if (deck) {
      deck.next();
    }

    if (index >= transactions.length) {
      handleReviewComplete();
    }
  }

  const handleBack = () => {
    if (deck) {
      deck.back();
    }
  }

  const handleReviewComplete = () => {
    isReviewComplete = true;
  }

  const getEmptyText = () => {
    if (transactions.length === 0) {
      return "All your transactions are categorized. Way to keep your ducks in a row!"
    }
    if (isReviewComplete) {
      return "You're done!"
    }
  }
</script>

{#snippet nextButton()}
  <Button
    onclick={handleNext}
    disabled={isAnimating || index >= transactions.length}
  >
    Next
  </Button>
{/snippet}

{#snippet backButton()}
  <Button
    onclick={handleBack}
    disabled={isAnimating || index == 0}
  >
    Back
  </Button>
{/snippet}

{#snippet card(transaction: TransactionWithAccount)}
  {#if !loadingCategories}
    <UncategorizedFlashcard
      transaction={transaction}
      categories={categories}
      {handleCategoryUpdate}
    />
  {/if}
{/snippet}

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
    {#if isReviewComplete || transactions.length === 0}
      <h3>{getEmptyText()}</h3>
    {:else}
      <div class="flashcard-container">
        <FlashcardDeck
          bind:this={deck}
          bind:index
          bind:isAnimating
          {transactions}
          {card}
          {nextButton}
          {backButton}
        />
      </div>
    {/if}
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
    padding-top: 100px;
  }
</style>
