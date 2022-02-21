import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";

export default function CommitsByDay({data}) {
    function bar_chart(element, data) {
        const chartDom = document.getElementById(element);
        const myChart = echarts.init(chartDom);
        let option;

        option = {
            xAxis: {
                type: 'time',
            },
            yAxis: {
                type: 'value'
            },
            series: [
                {
                    data: data,
                    type: 'bar',
                    step: 'start',
                    showSymbol: false,
                    itemStyle: {
                        color: '#2DA1EF'
                    }
                }
            ],
            grid: {
                left: 100,
                top: 50,
                right: 50,
                bottom: 50
            }
        };

        option && myChart.setOption(option);
        window.addEventListener('resize', function () {
            if (myChart != null) {
                myChart.resize();
            }
        });
    }

    useEffect(() => {
        bar_chart("total_commits_by_day", data)
    }, []); //
    return (
        <div className="row">
            <div className="col">
                <h2>Commits by day</h2>
                <div className="row">
                    <div className="col">
                        <div id="total_commits_by_day" className="chart"/>
                    </div>
                </div>
            </div>
        </div>
    )
}