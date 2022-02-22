import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";
import {line_chart} from "../../utils/echarts";

/*
option = {
  legend: {},
  tooltip: {},
  xAxis: [
    { type: 'time', gridIndex: 0 },
    { type: 'time', gridIndex: 1, axisLabel: {
      formatter: (function(value){
        return '';
      })
    }}
  ],
  yAxis: [{ gridIndex: 0, }, { gridIndex: 1, inverse:true, axisLabel: {
      formatter: (function(value){
        return value === 0 ? 0 : '-' + value;
      })
    } }],
  grid: [{ bottom: '50%' }, { top: '50%' }],
  series: [
    // These series are in the first grid.
    { type: 'bar', data:[['2021-02-1',10]] },
    // These series are in the second grid.
    { type: 'bar', xAxisIndex: 1, yAxisIndex: 1,  data:[['2021-02-1',10]] },
  ]
};
 */

function lines_added_vs_deleted(element, added, deleted) {
    const chartDom = document.getElementById(element);
    const myChart = echarts.init(chartDom);
    let option;

    option = {
        tooltip: {
            trigger: 'item'
        },
        xAxis: [{type: 'time', gridIndex: 0},
            {
                type: 'time', gridIndex: 1, axisLabel: {
                    formatter: (function (value) {
                        return '';
                    })
                }
            }],
        yAxis: [{gridIndex: 0, name: "Added"}, {
            gridIndex: 1, name: "Deleted", inverse: true, axisLabel: {
                formatter: (function (value) {
                    return value === 0 ? 0 : '-' + value;
                })
            }
        }],
        grid: [{bottom: '50%'}, {top: '50%'}],
        series: [
            {
                data: added,
                type: "bar",
                showSymbol: false,
                itemStyle: {
                    color: '#2DA1EF'
                }
            },
            {
                data: deleted,
                type: "bar",
                showSymbol: false,
                itemStyle: {
                    color: '#fd0e35'
                },
                xAxisIndex: 1,
                yAxisIndex: 1,
            }
        ],
    };

    option && myChart.setOption(option);
    window.addEventListener('resize', function () {
        if (myChart != null && myChart != undefined) {
            myChart.resize();
        }
    });
}

export default function LinesAddedDeleted({data}) {

    function lines_added_by_day_model() {
        return data.map((x) => {
            return [x[0], x[1] === 0 ? 1 : x[1]];
        })
    }

    function lines_deleted_by_day_model() {
        return data.map((x) => {
            return [x[0], x[2] === 0 ? 1 : x[2]];
        })
    }

    useEffect(() => {
        lines_added_vs_deleted("lines_added_deleted_by_day", lines_added_by_day_model(),
            lines_deleted_by_day_model());
    }, []);

    return (
        <div className="row">
            <div className="col">
                <h2>Lines Added vs. Deleted</h2>
                <div id="lines_added_deleted_by_day" className="col chart"/>
            </div>
        </div>
    )
}