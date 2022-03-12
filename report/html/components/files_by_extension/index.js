import {useEffect} from "preact/compat";
import * as echarts from "echarts";

export default function FilesByExtension({data}){
    function pie_chart_flower(element, data){
        var chartDom = document.getElementById(element);
        var myChart = echarts.init(chartDom);
        var option;

        option = {
            legend: {
                show:false
            },
            toolbox: {
                show: true,
                feature: {
                    mark: { show: true },
                    dataView: { show: true, readOnly: false },
                    restore: { show: true },
                    saveAsImage: { show: true }
                }
            },

            series: [
                {
                    name: 'File Extensions',
                    type: 'pie',
                    radius: '50%',
                    itemStyle: {
                        label: {
                            show: true,
                            formatter: 'BOOM',
                        },
                    },
                    data: data
                }
            ]
        };

        option && myChart.setOption(option);

        window.addEventListener('resize', function(){
            if(myChart != null && myChart != undefined){
                myChart.resize();
            }
        });
    }

    useEffect(() => {
        pie_chart_flower("files_by_extension", data);
    }, []);

    return (
        <div className="row">
            <div className="col">
                <h2>Files by extension</h2>
                <div id="files_by_extension" className="chart" />
            </div>
        </div>
    )
}