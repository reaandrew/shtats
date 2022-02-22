import * as echarts from "echarts";

export function bar_category(element, categories, data) {
    var chartDom = document.getElementById(element);
    var myChart = echarts.init(chartDom);
    var option;

    option = {
        xAxis: {
            type: 'category',
            data: categories
        },
        yAxis: {
            type: 'value'
        },
        series: [
            {
                data: data,
                type: 'bar',
                showBackground: true,
                backgroundStyle: {
                    color: 'rgba(180, 180, 180, 0.2)'
                },
                itemStyle: {
                    color: '#2DA1EF'
                }
            }
        ]
    };

    option && myChart.setOption(option);
    window.addEventListener('resize', function () {
        if (myChart != null) {
            myChart.resize();
        }
    });
}

export function line_chart(element, data){
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
                type: 'line',
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
    window.addEventListener('resize', function(){
        if(myChart != null && myChart != undefined){
            myChart.resize();
        }
    });
}