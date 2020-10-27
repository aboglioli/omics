import { ChartDataSets, ChartOptions, ChartType } from 'chart.js';
import { Label } from 'ng2-charts';

export class ChartDataClassPie {

  labels: Label[];
  values: number[];
  type: ChartType;
  legend: boolean;
  plugins: any[];
  colors: object[];
  options: ChartOptions;

}
export class ChartDataClassBar {

  labels: Label[];
  type: ChartType;
  legend: boolean;
  data: ChartDataSets[];
  plugins: any[];
  colors: object[];
  options: ChartOptions;

}
