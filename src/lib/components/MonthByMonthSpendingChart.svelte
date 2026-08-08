<!-- @component
  TODO: Add docstring
-->

<script lang="ts">
  import { Plot, Line, AxisX, AxisY, RuleX, RuleY, Pointer, HTMLTooltip, Dot } from "svelteplot";
    import SpendingChartTooltip from "./SpendingChartTooltip.svelte";

  // Arbitrary mock time-series data so the plot renders.
  const income = Array.from({ length: 12 }, (_, i) => {
    const Date_ = new Date(2020, 0, 1);
    Date_.setMonth(Date_.getMonth() + i);
    return {
      Date: Date_,
      Amount: Math.round(100 + i * 3 + Math.sin(i / 4) * 20),
    };
  })

  const spending = Array.from({ length: 12 }, (_, i) => {
    const Date_ = new Date(2020, 0, 1);
    Date_.setMonth(Date_.getMonth() + i);
    return {
      Date: Date_,
      Amount: Math.round(100 + i * 3 + Math.sin(i / 2) * 20),
    };
  });;

  const combinedData = income.map((data, idx) => { return { date: data.Date, income: data.Amount, spending: spending[idx].Amount }; });

  const formatDate = (date: Date) => { 
    const monthString = date.toLocaleString('default', { month: 'short' });
    return `${monthString} ${date.getFullYear()}`;
  }
</script>

<div class="container">
  <Plot grid subtitle="Month-by-month spending" height={320} marginTop={10} >
    <Line data={income} x="Date" y="Amount" stroke="#378727" strokeWidth={2} marker="circle-stroke" markerScale={0.75} />
    <Line data={spending} x="Date" y="Amount" stroke="#DC1716" strokeWidth={2} marker="circle-stroke" markerScale={0.75} />
    <AxisX
        data={income.map((d) => d.Date as Date)}
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
        data={combinedData}
        x="date"
        y="income">
        {#snippet children({ datum })}
            <div class="tooltip">
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
      data={income}
      x="Date"
      y="Amount"
      maxDistance={30}>
      {#snippet children({ data })}
          <RuleX {data} x="Date" opacity="0.3" />
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
    font-size: 12px;
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
