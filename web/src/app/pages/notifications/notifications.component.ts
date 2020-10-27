import { Component, OnInit } from '@angular/core';

import { INotification, INotificationBody } from '../../domain/models';
import { NotificationService } from '../../domain/services/notification.service';

@Component({
  selector: 'app-notifications',
  templateUrl: './notifications.component.html',
  styleUrls: ['./notifications.component.scss'],
})
export class NotificationsComponent implements OnInit {
  public notifications: INotification[] = [];

  constructor(
    private notificationService: NotificationService,
  ) {
    this.notificationService.getAll({}).subscribe(
      res => {
        this.notifications = res.notifications;
      },
      err => {
        console.log(err);
      }
    );

    setTimeout(() => {
      this.notificationService.markAllAsRead().subscribe();
    }, 1000);
  }

  ngOnInit(): void {}

  makeText(notification: INotification): string {
    const body = notification.body;

    switch(notification.code) {
      case 'welcome': {
        return `
          Bienvenido a <b>Omics</b>
        `;
      }
      case 'new-publication-from-followed-author': {
        return `
          El author
          <a href="/profile/${body.author_id}">
            ${body.author_username}
          </a>
          , a quien sigues, publicó una nueva obra que podría interesarte:
          <a href="/read/${body.publication_id}">
            ${body.publication_name}
          </a>.
        `
      }
      case 'publication-approved': {
        return `
          ¡Tu publicación
          <a href="/deskboard">
            ${body.publication_name}
          </a>
          fue <b>aprobada</b>!<br>
          De ahora en más aparecerá en el catálogo.
        `
      }
      case 'publication-rejected': {
        return `
          Lo sentimos. Tu publicación
          <a href="/deskboard">
            ${body.publication_name}
          </a>
          fue <b>rechazada</b>.
        `
      }
      case 'publication-liked': {
        return `
          La publicación
          <a href="/deskboard">
            ${body.publication_name}
          </a>
          recibió un <b>like</b> del usuario
          <b>${body.reader_username}</b>
        `
      }
      case 'author-followed': {
        return `
          El usuario <b>${body.reader_username}</b> comenzó a seguirte.
        `;
      }
      default: {
        return `
          Desconocida
        `;
      }
    }
  }

  getClass(notification: INotification): string {
    let classes = [];

    if (notification.read) {
      classes.push('read');
    } else {
      classes.push('unread');
    }

    const greenCodes = [
      'publication-approved',
      'subscription-activated',
      'contract-approved',
      'donation-received',
    ];
    const redCodes = [
      'publication-rejected',
      'contract-rejected',
    ];

    if (greenCodes.includes(notification.code)) {
      classes.push('green');
    } else if (redCodes.includes(notification.code)) {
      classes.push('red');
    }

    return classes.join(' ');
  }
}

// const exampleNotifications: INotification[] = [
//   {
//     id: 'notification-1',
//     user_id: 'user-1',
//     code: 'welcome',
//     body: { },
//     read: false,
//   }, {
//     id: 'notification-2',
//     user_id: 'user-1',
//     code: 'new-publication-from-followed-author',
//     body: {
//       publication_id: 'publication-1',
//       publication_name: 'The Nombre',
//       author_id: 'author-1',
//       author_username: 'user32',
//       author_name: 'ElUsuario',
//       author_lastname: 'SoyYo',
//     },
//     read: false,
//   }, {
//     id: 'notification-2',
//     user_id: 'user-1',
//     code: 'publication-approved',
//     body: {
//       publication_id: 'publication-1',
//       publication_name: 'The Nombre',
//     },
//     read: false,
//   }, {
//     id: 'notification-2',
//     user_id: 'user-1',
//     code: 'publication-rejected',
//     body: {
//       publication_id: 'publication-1',
//       publication_name: 'The Nombre',
//     },
//     read: false,
//   }, {
//     id: 'notification-2',
//     user_id: 'user-1',
//     code: 'publication-liked',
//     body: {
//       publication_id: 'publication-1',
//       publication_name: 'The Nombre',
//       reader_id: 'reader-1',
//       reader_username: 'The Nombre',
//     },
//     read: true,
//   }, {
//     id: 'notification-2',
//     user_id: 'user-1',
//     code: 'author-followed',
//     body: {
//       reader_id: 'reader-1',
//       reader_username: 'reeeeader',
//       reader_name: 'Reader',
//       reader_lastname: 'Lastname',
//     },
//     read: true,
//   }
// ];
