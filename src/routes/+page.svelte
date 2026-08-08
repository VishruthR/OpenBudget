<script lang="ts">
  import Button from "$lib/components/Button.svelte";
    import MonthByMonthSpendingChart from "$lib/components/MonthByMonthSpendingChart.svelte";
  import TransactionsTable from "$lib/components/TransactionsTable.svelte";
  import { Tooltip } from "bits-ui";
 </script>

<main class="container">
  <MonthByMonthSpendingChart />
  <TransactionsTable />
    <Tooltip.Root delayDuration={200}>
    <Tooltip.Trigger
      class="border-border-input bg-background-alt shadow-btn ring-dark ring-offset-background
		hover:bg-muted focus-visible:ring-dark focus-visible:ring-offset-background focus-visible:outline-hidden inline-flex size-10 items-center justify-center rounded-full border focus-visible:ring-2 focus-visible:ring-offset-2"
    >
      <p>test</p>
    </Tooltip.Trigger>
    <Tooltip.Content
      sideOffset={8}
      class="animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 origin-(--bits-tooltip-content-transform-origin)"
    >
      <div
        class="rounded-input border-dark-10 bg-background shadow-popover outline-hidden z-0 flex items-center justify-center border p-3 text-sm font-medium"
      >
        Make some magic!
      </div>
    </Tooltip.Content>
  </Tooltip.Root>
</main>

<style>
  .container {
    min-height: 100vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 32px;
    gap: 48px;
  }
</style>
