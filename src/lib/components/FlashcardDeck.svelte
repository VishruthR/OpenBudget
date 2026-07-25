<!-- @component
  Displays a deck of transaction flashcards with discard/accept actions.
  Shows up to 3 cards stacked, with controls to process each card.
-->

<script lang="ts">
  import type { TransactionWithAccount } from "$lib/types";
  import type { Snippet } from "svelte";

  interface Props {
    transactions: TransactionWithAccount[];
    currentIndex: number;
    throwDirection: "left" | "right" | null;
    reverse: boolean | null;
    card: Snippet<[TransactionWithAccount]>;
    nextButton: Snippet<[]>;
    backButton: Snippet<[]>;
    onAnimationEnd: () => void;
    onComplete: () => void;
  }

  let {
    transactions,
    currentIndex = $bindable(0), 
    throwDirection = $bindable(null),
    reverse = $bindable(null),
    card,
    nextButton,
    backButton,
    onAnimationEnd,
    onComplete,
  }: Props = $props();
  
  // Should accept currentIndex, throwDirection, and Reverse?
  // use $effect to listen to changes on any of the three above (they should always change in coordination) to update and play animation?
  // User should also just pass in left and right buttons as snippert bh
  // have to update currentIndex after animations are over, so you should include that as a callback
  // TODO: Have claude write a docstring on how to use this component
  let totalCards = $derived(transactions.length);
  let currentTransaction = $derived(
    currentIndex < totalCards ? transactions[currentIndex] : null
  );
  let isComplete = $derived(currentIndex >= totalCards);

  // Get up to 3 cards to display
  let visibleCards = $derived.by(() => {
    const cards: TransactionWithAccount[] = [];
    for (let i = 0; i < 3 && currentIndex + i < totalCards; i++) {
      cards.push(transactions[currentIndex + i]);
    }
    return cards;
  });

  function handleAnimationEnd() {
    if (throwDirection && currentTransaction) {
      throwDirection = null;
      reverse = null;

      checkComplete();
      onAnimationEnd();
    }
  }
  
  function checkComplete() {
    if (currentIndex >= totalCards) {
      onComplete();
    }
  }
</script>

<div class="flashcard-deck">
  <div class="cards-container">
    {#if !isComplete}
      {#each visibleCards as visibleCard, index (`${visibleCard.transaction.name}-${visibleCard.transaction.date}-${visibleCard.transaction.amount}-${index}`)}
        <div
          class="card-wrapper"
          class:card-back-2={index === 2}
          class:card-back-1={index === 1}
          class:card-front={index === 0}
          class:throw-left={index === 0 && throwDirection === "left" && !reverse}
          class:throw-right={index === 0 && throwDirection === "right" && !reverse}
          class:recover-left={index === 0 && throwDirection === "left" && !!reverse}
          class:recover-right={index === 0 && throwDirection === "right" && !!reverse}
          onanimationend={index === 0 ? handleAnimationEnd : undefined}
        >
          {@render card(visibleCard)}
        </div>
      {/each}
    {/if}
  </div>

  <div class="controls">
    {@render backButton()}
    
    <span class="counter">
      {isComplete ? totalCards : currentIndex + 1}/{totalCards}
    </span>

    {@render nextButton()}
  </div>
</div>

<style>
  .flashcard-deck {
    display: flex;
    flex-direction: column;
    align-items: center;
    flex: 1;
    gap: 24px;
  }

  .cards-container {
    position: relative;
    width: 100%;
    height: 280px;
  }

  .card-wrapper {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    transition: transform 0.2s ease-out;
    box-shadow: 0 4px 20px 0 rgba(0, 0, 0, 0.3);
    border-radius: 12px;
  }

  .card-front {
    z-index: 3;
    transform: translateY(0);
  }

  .card-back-1 {
    z-index: 2;
    transform: translateY(-16px);
  }

  .card-back-2 {
    z-index: 1;
    transform: translateY(-32px);
  }

  .controls {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    max-width: 400px;
    gap: 16px;
  }

  .counter {
    font-size: 16px;
    font-weight: 400;
    color: var(--grey-300);
  }

  /* Throw animations */
  @keyframes throw-left {
    0% {
      transform: translateY(0) rotate(0deg);
      opacity: 1;
    }
    100% {
      transform: translate(-150px, -100px) rotate(-15deg);
      opacity: 0;
    }
  }

  @keyframes throw-right {
    0% {
      transform: translateY(0) rotate(0deg);
      opacity: 1;
    }
    100% {
      transform: translate(150px, -100px) rotate(15deg);
      opacity: 0;
    }
  }

  .throw-left {
    animation: throw-left 0.35s ease-out forwards;
    z-index: 10;
  }

  .throw-right {
    animation: throw-right 0.35s ease-out forwards;
    z-index: 10;
  }
  
  /* Recover animations */
  @keyframes recover-left {
    0% {
      transform: translate(-150px, -100px) rotate(-15deg);
      opacity: 0;
    }
    100% {
      transform: translateY(0) rotate(0deg);
      opacity: 1;
    }
  }

  @keyframes recover-right {
    0% {
      transform: translate(150px, -100px) rotate(15deg);
      opacity: 0;
    }
    100% {
      transform: translateY(0) rotate(0deg);
      opacity: 1;
    }
  }

  .recover-left {
    animation: recover-left 0.35s ease-out forwards;
    z-index: 10;
  }

  .recover-right {
    animation: recover-right 0.35s ease-out forwards;
    z-index: 10;
  }
</style>
