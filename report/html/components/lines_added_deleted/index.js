import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";
import {line_chart} from "../../utils/echarts";

function lines_added_vs_deleted(element, added, deleted) {
    const chartDom = document.getElementById(element);
    const myChart = echarts.init(chartDom);
    let option;
    const no_label = () => {

    }
    option = {
        tooltip: {
            trigger: 'item'
        },
        xAxis: [{type: 'time', gridIndex: 0, axisLabel: no_label},
            {
                type: 'time', gridIndex: 1
            }],
        yAxis: [{gridIndex: 0, name: "Added", type: 'log'}, {
            type: 'log',
            gridIndex: 1,
            name: "Deleted",
            inverse: true,
            nameGap: -30,
            nameTextStyle: {
                padding: [60,0,0,0]
            },
            axisLabel: {
                formatter: (function (value) {
                    return value === 0 ? 0 : '-' + value;
                })
            }
        }],
        grid: [{
            left: 100,
            right: 50,
            bottom: '50%'
        }, {
            left: 100,
            right: 50,
            top: '50%'
        }],
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
    function group_by_months(data){
        let obj = data.reduce((prev, current) => {
            const date =  new Date(current[0])
            const lastDayOfMonth = new Date(date.getFullYear(), date.getMonth()+1, 0)
            let key = lastDayOfMonth.toISOString().split('T')[0]
            if (prev[key] === undefined){
                prev[key] = {added:1, deleted:1};
            }
            prev[key].added += current[1];
            prev[key].deleted += current[2];
            return prev;
        }, {});
        return Object.keys(obj).map(x=>[x, obj[x].added, obj[x].deleted]);
    }
    function lines_added_by_day_model() {
        if (data.length > 360){
            return group_by_months(data);
        }else{
            return data.map((x) => {
                return [x[0], x[1] === 0 ? 1 : x[1]];
            })
        }
    }

    function lines_deleted_by_day_model() {
        if (data.length > 360){
            return group_by_months(data);
        }else {
            return data.map((x) => {
                return [x[0], x[2] === 0 ? 1 : x[2]];
            })
        }
    }

    useEffect(() => {
        lines_added_vs_deleted("lines_added_deleted_by_day",
            lines_added_by_day_model(),
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
