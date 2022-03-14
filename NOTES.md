# Notes

## Get the sizes of blobs

```shell
git rev-list --all --objects | \
  git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' | \
  sort --human-numeric-sort -k3nr | \
  head | \
  numfmt --field 3 --to=iec
```

## Heat map for commits

[Based on this link from echarts but with modifications for horizontal and spacing](https://echarts.apache.org/examples/en/editor.html?c=calendar-vertical)

```javascript
import * as echarts from 'echarts';

var chartDom = document.getElementById('main');
var myChart = echarts.init(chartDom);
var option;

function getVirtulData(year) {
  year = year || '2017';
  var date = +echarts.number.parseDate(year + '-01-01');
  var end = +echarts.number.parseDate(+year + 1 + '-01-01');
  var dayTime = 3600 * 24 * 1000;
  var data = [];
  for (var time = date; time < end; time += dayTime) {
    data.push([
      echarts.format.formatTime('yyyy-MM-dd', time),
      Math.floor(Math.random() * 1000)
    ]);
  }
  return data;
}
option = {
  tooltip: {
    position: 'top',
    formatter: function (p) {
      var format = echarts.format.formatTime('yyyy-MM-dd', p.data[0]);
      return format + ': ' + p.data[1];
    }
  },
  visualMap: {
    min: 0,
    max: 1000,
    calculable: true,
    orient: 'horizontal',
    left: 0,
    top: '',
    color: ['#2DA1EF', '#C7DBFF']
  },
  calendar: [
    {
      top: 100,
      orient: 'horizontal',
      height: 100,
      cellSize: ['auto', 10],
      range: '2015'
    },
    {
      height: 100,
      top: 230,
      cellSize: ['auto', 10],
      orient: 'horizontal',
      range: '2016'
    },
    {
      height: 100,
      top: 360,
      cellSize: ['auto', 10],
      orient: 'horizontal',
      range: '2017',
      dayLabel: {
        margin: 5
      }
    }
  ],
  series: [
    {
      type: 'heatmap',
      coordinateSystem: 'calendar',
      calendarIndex: 0,
      data: getVirtulData('2015')
    },
    {
      type: 'heatmap',
      coordinateSystem: 'calendar',
      calendarIndex: 1,
      data: getVirtulData('2016')
    },
    {
      type: 'heatmap',
      coordinateSystem: 'calendar',
      calendarIndex: 2,
      data: getVirtulData('2017')
    }
  ]
};

option && myChart.setOption(option);

```