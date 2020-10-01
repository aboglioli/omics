import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'truncateString'
})
export class TruncateStringPipe implements PipeTransform {

  // para devolver la cantidad de characters según lo indicado (limitado por 50 por defecto y sin palabras completadas en falso por defecto)
  transform(value: string, limit = 50, completeWords = false, end = '...'): string {
    if ( value.length < limit ) {

      return value;

    }

    if (  completeWords ) {

      limit = value.substr(0, limit).lastIndexOf( ' ' ); // Da la última palabra completa

    }

    return `${value.substring(0, limit)}${end}`;
  }

}
