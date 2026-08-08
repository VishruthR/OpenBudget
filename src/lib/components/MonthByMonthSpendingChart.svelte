<!-- @component
  TODO: Add docstring
-->

<script lang="ts">
  import { Plot, Line, AxisX, AxisY, RuleX, RuleY, Pointer } from "svelteplot";

  // Arbitrary mock time-series data so the plot renders.
  const income = Array.from({ length: 12 }, (_, i) => {
    const Date_ = new Date(2020, 0, 1);
    Date_.setMonth(Date_.getMonth() + i);
    return {
      Date: Date_,
      Amount: 100 + i * 3 + Math.sin(i / 4) * 20,
    };
  })

  const spending = Array.from({ length: 12 }, (_, i) => {
    const Date_ = new Date(2020, 0, 1);
    Date_.setMonth(Date_.getMonth() + i);
    return {
      Date: Date_,
      Amount: 100 + i * 3 + Math.sin(i / 2) * 20,
    };
  });;

  // Gets y axis ticks
  // Tick marks are based on the maximum value in income and spending's order of magnitude / 2
  // e.g. If highest value is 5,232 -> ticks are indexed by 1,000 / 2 = 500
  // const getAxisYTicks = () => {
  //   const maxValue = Math.max(...income.map((v) => v.Amount), ...spending.map((v) => v.Amount));
  //   const orderOfMagnitude = Math.floor(Math.log10(maxValue));
  //   const increment = Math.pow(10, orderOfMagnitude) / 2;
  //   const nearestGreaterIncrement = increment * (Math.floor(maxValue / increment) + 1);
  //   const tickInterval = nearestGreaterIncrement / 5;
  //   const ticks = Array(5 + 1).keys().map((i) => i * tickInterval);
  //
  //   return [...ticks];
  // }

  const formatDate = (date: Date) => { 
    const monthString = date.toLocaleString('default', { month: 'short' });
    return `${monthString} ${date.getFullYear()}`;
  }
</script>

<div class="container">
  <Plot grid subtitle="Month-by-month spending" marginTop={10} >
    <Line data={income} x="Date" y="Amount" stroke="#378727" strokeWidth={2} marker="circle-stroke" markerScale={0.75} />
    <Line data={spending} x="Date" y="Amount" stroke="#DC1716" strokeWidth={2} marker="circle-stroke" markerScale={0.75} />
    <Pointer
      data={income}
      x="Date"
      y="Amount"
      maxDistance={30}>
      {#snippet children({ data })}
          <RuleX {data} x="Date" opacity="0.3" />
          <AxisX
              tickFontSize={9} 
              data={data.map((d) => d.Date as Date)}
              tickFormat={(d) => formatDate(d as Date)} />
          <AxisY
              title=""
              tickCount={6} 
              tickFormat={(t) => `$${t as number}`}
          />
      {/snippet}
    </Pointer>
  </Plot>
</div>

<style>
  .container {
    width: 700px;
    border: 2px solid var(--grey-500);
    border-radius: 8px;
    padding: 10px;
  }
</style>
