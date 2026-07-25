<script lang="ts">
  import { onMount } from "svelte";
    import { categoriesApi } from "$lib/api/categories";
  import type { CategoryOverview, TransactionWithAccount } from "$lib/types";
    import FlashcardDeck from "$lib/components/FlashcardDeck.svelte";
    import { transactionsApi } from "$lib/api/transactions";
    import UncategorizedFlashcard from "$lib/components/UncategorizedFlashcard.svelte";
    import Button from "$lib/components/Button.svelte";

  let currentIndex = $state(0);
  let nextIndexIncrement = $state(0);
  let isAnimating = $state(false);
  let throwDirection = $state<"left" | "right" | null>(null);
  let lastThrowDirection = $state<"left" | "right" | null>(null);
  let reverse = $state<boolean | null>(null);
  
  let transactions = $state<TransactionWithAccount[]>([]);
  async function loadTransactions() {
    transactions = await transactionsApi.getTransactionsByCategory("Uncategorized");
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
    transactionsApi.updateTransactionCategory(transactionId, categoryId);
    handleNext();
    console.log("updated!");
  }

  const handleAnimationEnd = () => {
    isAnimating = false;
    lastThrowDirection = throwDirection;
    throwDirection = null;
    reverse = null;
    currentIndex += nextIndexIncrement;
    nextIndexIncrement = 0;
    console.log("done animating");
  }

  const handleNext = () => {
    isAnimating = true;

    // Alternate throwing directions
    if (lastThrowDirection == "right") {
      throwDirection = "left";
    } else {
      throwDirection = "right";
    }

    reverse = false;
    nextIndexIncrement = 1;
    console.log("accepted!");
  }

  const handleBack = () => {
    if (currentIndex === 0) return;
    isAnimating = true;

    // TODO: Bug, if they spam back, then the cards will all come from the same direction
    throwDirection = lastThrowDirection;
    reverse = true;

    currentIndex -= 1;
    console.log("discarded");
  }

  const handleReviewComplete = () => {
    console.log("review complete");
  }
</script>

{#snippet nextButton()}
  <Button
    onclick={handleNext}
    disabled={isAnimating}
  >
    Next
  </Button>
{/snippet}

{#snippet backButton()}
  <Button
    onclick={handleBack}
    disabled={isAnimating}
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
    <div class="flashcard-container">
      <FlashcardDeck
        bind:currentIndex
        {transactions}
        bind:throwDirection
        bind:reverse
        {card}
        {nextButton}
        {backButton}
        onAnimationEnd={handleAnimationEnd}
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
