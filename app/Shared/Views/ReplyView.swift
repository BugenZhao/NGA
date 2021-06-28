//
//  ReplyView.swift
//  NGA
//
//  Created by Bugen Zhao on 6/28/21.
//

import Foundation
import SwiftUI
import RemoteImage

struct ReplyView: View {
  let reply: Reply
  let user: User?

  init(reply: Reply) {
    self.reply = reply
    self.user = try! (logicCall(.localUser(.with { $0.userID = reply.authorID })) as LocalUserResponse).user
  }

  var body: some View {
    let user = self.user

    VStack(alignment: .leading, spacing: 8) {
      HStack {
        avatar
          .frame(width: 32, height: 32)
          .clipShape(Circle())
        VStack(alignment: .leading) {
          Text(user?.name ?? reply.authorID)
            .font(.subheadline)
          HStack {
            Text("#\(reply.floor) · " + timeago(reply.postDate))
          } .font(.footnote)
            .foregroundColor(.secondary)
        }
        Spacer()
        if reply.score > 0 {
          HStack {
            Text("\(reply.score)")
              .fontWeight(.medium)
            Image(systemName: "chevron.up")
          } .font(.callout)
        }
      }

      ReplyContentView(spans: reply.content.spans)
    } .padding(.vertical, 4)
      .contextMenu {
      Button(action: { copyContent(reply.content.raw) }) {
        Label("Copy Raw Content", systemImage: "doc.on.doc")
      }
    }
  }

  var avatar: AnyView {
    let placeholder = Image(systemName: "person.circle.fill").resizable()

    if let url = URL(string: user?.avatarURL ?? "") {
      return AnyView(RemoteImage(
        type: .url(url),
        errorView: { _ in placeholder },
        imageView: { $0.resizable() },
        loadingView: { placeholder }
        ))
    } else {
      return AnyView(placeholder)
    }
  }

  func copyContent(_ content: String) {
    #if os(iOS)
      UIPasteboard.general.string = content
    #elseif os(macOS)
      let pb = NSPasteboard.general
      pb.clearContents()
      pb.writeObjects([content as NSString])
    #endif
  }
}
