<!-- @component
  TODO: Add docstring
-->

<script lang="ts">
    import { transactionsApi } from "$lib/api/transactions";
    import { onMount } from "svelte";
  import { Plot, Line, AxisX, AxisY, RuleX, Pointer, HTMLTooltip } from "svelteplot";

  interface DatedAmount {
    date: Date;
    amount: number;
  }

  let income = $state<DatedAmount[]>([]);
  let spending = $state<DatedAmount[]>([]);

  const today = new Date();
  let lastYear = new Date();
  lastYear.setFullYear(lastYear.getFullYear() - 1);

  const loadMonthlyStats = async () => {
    const [monthlyIncome, monthlySpending] = await transactionsApi.getMonthlyIncomeAndSpending(lastYear.toISOString().slice(0, 10), today.toISOString().slice(0, 10));

    income = monthlyIncome.map((m) => {
      return {
        date: new Date(m.month),
        amount: m.amount
      }
    })

    spending = monthlySpending.map((m) => {
      return {
        date: new Date(m.month),
        amount: m.amount
      }
    })
  }
  onMount(loadMonthlyStats);

  // This array enables us to render the same tooltip for the income and spending lines
  const tooltipAnchors = $derived.by(() => {
    const combinedData = income.map((data, idx) => { return { date: data.date, income: data.amount, spending: spending[idx].amount }; });

    return combinedData.flatMap((d) => [
      { ...d, amount: d.income },
      { ...d, amount: d.spending },
    ]);
  });

  const formatDate = (date: Date) => {
    const monthString = date.toLocaleString('default', { month: 'short' });
    return `${monthString} ${date.getFullYear()}`;
  }

  const monthTimestamps = $derived(income.map((d) => d.date.getTime()));
  const amounts = $derived([...income.map((d) => d.amount), ...spending.map((d) => d.amount)]);
  const minAmount = $derived(Math.min(...amounts));
  const maxAmount = $derived(Math.max(...amounts));

  const getTooltipPosition = (datum: { date?: Date; amount?: number }): string => {
    const date = datum?.date;
    if (!date) return "tooltip-right";

    const idx = monthTimestamps.indexOf(date.getTime());
    const horizontal = idx >= monthTimestamps.length - 3 ? "tooltip-left" : "tooltip-right";

    const fraction = ((datum.amount ?? minAmount) - minAmount) / (maxAmount - minAmount || 1);
    const vertical = fraction > 0.7 ? "tooltip-bottom" : fraction < 0.3 ? "tooltip-top" : "";

    return `${horizontal} ${vertical}`;
  }

  // const mappedIncome = $derived(income.map(i => Object.fromEntries(i)));
  const incomeRecord = $derived(income.map((i) => {
    return {
      "date": i.date,
      "amount": i.amount,
    }
  }));

  const spendingRecord = $derived(spending.map((i) => {
    return {
      "date": i.date,
      "amount": i.amount,
    }
  }))
</script>

<div class="container">
  <Plot grid subtitle="Month-by-month spending" height={320} marginTop={10} >
    <Line data={incomeRecord} x="date" y="amount" stroke="#378727" strokeWidth={2} marker="circle-stroke" markerScale={0.75} />
    <Line data={spendingRecord} x="date" y="amount" stroke="#DC1716" strokeWidth={2} marker="circle-stroke" markerScale={0.75} />
    <AxisX
        data={income.map((d) => d.date as Date)}
        tickFontSize={9} 
        tickSize={0}
        tickPadding={10}
        tickFormat={(d) => formatDate(d as Date)} />
    <AxisY
        title=""
        tickCount={6} 
        tickPadding={10}
        tickFormat={(t) => `$${t as number}`}
        stroke="#AEAEAE"
    />
    {#snippet overlay()}
      <HTMLTooltip
        data={tooltipAnchors}
        x="date"
        y="amount">
        {#snippet children({ datum })}
            <div class={`tooltip ${getTooltipPosition(datum)}`}>
                <div class="tooltip-header">
                  <p class="tooltip-header-text">{datum.date ? formatDate(datum.date) : "placeholder"}</p>
                </div>
                <div class="tooltip-data">
                  <div class="tooltip-data-line">
                    <p class="tooltip-data-text">Income</p>
                    <p class="tooltip-data-text income">+${datum.income}</p>
                  </div>
                  <div class="tooltip-data-line">
                    <p class="tooltip-data-text">Spending</p>
                    <p class="tooltip-data-text spending">-${datum.spending}</p>
                  </div>
                  <hr class="tooltip-divider">
                  <div class="tooltip-data-line">
                    <p class="tooltip-data-text">Savings</p>
                    <p class="tooltip-data-text savings">{datum.income - datum.spending >= 0 ? "+" : ""}${datum.income - datum.spending}</p>
                  </div>
                </div>
            </div>
        {/snippet}
      </HTMLTooltip>
    {/snippet}
    <Pointer
      data={tooltipAnchors}
      x="date"
      y="amount"
      maxDistance={30}>
      {#snippet children({ data })}
          <RuleX {data} x="date" opacity="0.5" stroke="#535353" />
      {/snippet}
    </Pointer>
  </Plot>
</div>

<style>
  .container {
    position: relative;
    width: 700px;
    border: 2px solid var(--grey-500);
    border-radius: 8px;
    padding: 10px;
  }

  .tooltip {
    background-color: var(--bg-white);
    border-radius: 10px;
    line-height: 1.2;
    box-shadow:
        rgba(0, 0, 0, 0.25) 0px 0px 10px 0px;
    transform: translate(var(--tooltip-tx, 10px), var(--tooltip-ty, -50%));
  }

  .tooltip-left {
    --tooltip-tx: calc(-100% - 10px);
  }

  .tooltip-right {
    --tooltip-tx: 10px;
  }

  .tooltip-top {
    --tooltip-ty: calc(-100% - 10px);
  }

  .tooltip-bottom {
    --tooltip-ty: 10px;
  }

  .tooltip-header {
    display: flex;
    justify-content: center;
    width: 100%;
    background-color: var(--grey-200);
  }

  .tooltip-header-text {
    color: var(--bg-white);
    font-size: 18;
    font-weight: bold;
    margin-top: 8px;
    margin-bottom: 5px;
  }

  .tooltip-data {
    font-size: 13px;
    color: var(--grey-200);
    min-width: 120px;
    margin: 3px 3px 3px 3px;
    padding-bottom: 5px;
  }

  .tooltip-data-line {
    display: flex;
    justify-content: space-between;
  }

  .tooltip-data-text {
    margin: 1px 0px;
  }
  
  .tooltip-divider {
    height: 1px;
    background-color: var(--grey-200);
    border: none;
  }

  .income {
    color: var(--profit-green);
  }

  .spending {
    color: var(--loss-red);
  }

  .savings {
    color: var(--savings-purple);
  }
</style>
