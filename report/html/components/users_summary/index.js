import {h} from 'preact';
import * as echarts from 'echarts';
import {useEffect} from "preact/compat";
import {line_chart} from "../../utils/echarts";

export default function UserSummary({id, data}) {

    function autoFontSize(element) {
        let width = document.getElementById(element).offsetWidth;
        let newFontSize = Math.round(width / 11);
        return newFontSize;
    };

    function nFormatter(num, digits) {
        const lookup = [
            { value: 1, symbol: "" },
            { value: 1e3, symbol: "K" },
            { value: 1e6, symbol: "M" },
            { value: 1e9, symbol: "B" },
            { value: 1e12, symbol: "T" }
        ];
        const rx = /\.0+$|(\.[0-9]*[1-9])0+$/;
        var item = lookup.slice().reverse().find(function(item) {
            return num >= item.value;
        });
        return item ? (num / item.value).toFixed(digits).replace(rx, "$1") + item.symbol : "0";
    }

    function chart_it(element) {
        var chartDom = document.getElementById(element);
        var myChart = echarts.init(chartDom);
        var option;

        option = {
            grid: {
                top: 20,
                left: 50,
                right:10
            },
            xAxis: {
                type: 'time'
            },
            yAxis: {
                type: 'log',
                axisLabel: {
                    formatter: (d) => {
                        return nFormatter(d,2);
                    }
                }
            },
            series: [
                {
                    data: data.commits,
                    type: 'line',
                    step: 'start',
                    smooth: true,
                    showSymbol: false,
                    animation: false,
                    itemStyle: {
                        normal: {
                            color: '#2DA1EF',
                            lineStyle: {
                                color: '#2DA1EF'
                            },
                            areaStyle: {
                                color: '#2DA1EF'
                            },

                        }
                    }
                }
            ]
        };

        option && myChart.setOption(option);

        window.addEventListener('resize', function () {
            if (myChart != null && myChart != undefined) {
                myChart.resize();
            }
        });
    }

    useEffect(() => {
        chart_it("user_commits_" + id);
    }, []);


    return (
        <div>
            <div className="row">
                <div className="col-auto">
                    <img src={`https://www.gravatar.com/avatar/${data.gravatar}?s=50`}/>
                </div>
                <div className="col  small">
                    <div className="row">
                        <div className="col">
                            {data.name}
                        </div>
                    </div>
                    <div className="row">
                        <div className="col-auto">
                            <span className="me-4">{data.total_commits} commits</span>
                        </div>
                    </div>
                    <div className="row">
                        <div className="col">

                            <span className="text-success me-2">{data.lines_added} ++</span>
                            <span className="text-danger">{data.lines_deleted} --</span>
                        </div>
                    </div>
                </div>
                <div className="col-auto small fw-bold">
                    #{data.index}
                </div>
            </div>
            <div className="row">
                <div className="col user-chart" id={"user_commits_" +id}>
                </div>
            </div>
        </div>
    )
}