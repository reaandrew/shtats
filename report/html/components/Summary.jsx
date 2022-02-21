import { h } from 'preact';

export function Summary({data}) {

    let items = (data.map(x=> (
         <tr>
             <td>{x[0]}</td>
             <td>{x[1]}</td>
         </tr>
    )));
    return (
        <table className="table">
            <thead>
            <tr>
                <th>Item</th>
                <th>Summary</th>
            </tr>
            </thead>
            <tbody>
            {items}
            </tbody>
        </table>
    );
}