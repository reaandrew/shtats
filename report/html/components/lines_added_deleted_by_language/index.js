import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";
import {line_chart} from "../../utils/echarts";


export default function LinesAddedDeletedByLanguage({data}) {
    console.log('DATA for LinesAddedDeletedByLanguage', data)
    function draw(s, data) {
        var chartDom = document.getElementById(s);
        var myChart = echarts.init(chartDom);
        var option;

        let xAxisData = [];
        let data1 = [];
        let data2 = [];

        data.sort((a, b) => {
            return b[2] + b[1] > a[2] + a[1] ? 1 // if b should come earlier, push a to end
                : b[2] + b[1] < a[2] + a[1] ? -1 // if b should come later, push a to begin
                    : 0;
        }).forEach(value => {
            xAxisData.push(value[0]);
            data1.push(value[1]);
            data2.push(value[2]);
        })
        option = {
            legend: {
                data: ['added', 'deleted'],
                left: '10%'
            },
            tooltip: {},
            xAxis: {
                data: xAxisData,
                axisLine: {onZero: true},
                splitLine: {show: false},
                splitArea: {show: false}
            },
            yAxis: {type: 'log',},
            grid: {
                left: 100,
                top: 50,
                right: 50,
                bottom: 50
            },
            series: [
                {
                    name: 'added',
                    type: 'bar',
                    stack: 'one',
                    data: data1,
                    itemStyle: {
                        color: '#2DA1EF'
                    }
                },
                {
                    name: 'deleted',
                    type: 'bar',
                    stack: 'one',
                    data: data2,
                    itemStyle: {
                        color: '#fd0e35'
                    },
                }
            ]
        };

        myChart.setOption({
            title: {
                bottom: 0,
                right: '10%',
                width: 100,
                textStyle: {
                    fontSize: 12,
                    color: '#fff'
                }
            }
        });

        option && myChart.setOption(option);
    }

    useEffect(() => {
        draw("lines_added_deleted_by_language", data);
    }, []);

    return (
        <div className="row">
            <div className="col">
                <h2>Average Lines Added vs. Deleted by Language per commit</h2>
                <div id={"lines_added_deleted_by_language"} className="col chart"/>
            </div>
        </div>
    )
}