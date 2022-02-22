import {h} from 'preact';
import * as echarts from "echarts";
import {useEffect} from "preact/compat";
import {line_chart} from "../../utils/echarts";

function lines_added_vs_deleted(element, added, deleted){
    const chartDom = document.getElementById(element);
    const myChart = echarts.init(chartDom);
    let option;

    option = {
        tooltip: {
            trigger: 'item'
        },
        xAxis: {
            type: 'time',
        },
        yAxis: {
            type: 'log'
        },
        dataZoom: [
            {
                show: true,
                realtime: true
            },
        ],
        series: [
            {
                data: added,
                type: "bar",
                stack: 'Total',
                showSymbol: false,
                itemStyle: {
                    color: '#2DA1EF'
                }
            },
            {
                data: deleted,
                type: "bar",
                stack: 'Total',
                showSymbol: false,
                itemStyle: {
                    color: '#fd0e35'
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
    window.addEventListener('resize', function(){
        if(myChart != null && myChart != undefined){
            myChart.resize();
        }
    });
}

export default function LinesAddedDeleted({data}){

    function lines_added_by_day_model(){
        return data.map((x) => {
            return [x[0], x[1]];
        })
    }

    function lines_deleted_by_day_model(){
        return data.map((x) => {
            return [x[0], x[2]];
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
                <div id="lines_added_deleted_by_day" className="col chart" />
            </div>
        </div>
    )
}