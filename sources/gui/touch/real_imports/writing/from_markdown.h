#pragma once
#include <QList>
#include <QString>
#include <QTextBlock>
#include <memory> // for std::unique_ptr

namespace Markdown::From
{

enum class TokenType
{
    None,
    BOLD_START,
    BOLD_END,
    ITALIC_START,
    ITALIC_END,
    LITERAL_ASTERISK,
    UNDERLINE_START,
    UNDERLINE_END,
    LITERAL_UNDERSCORE,
    STRIKETHROUGH_START,
    STRIKETHROUGH_END,
    LITERAL_TILDE,
    IMAGE_START,
    IMAGE_ALT_TEXT,
    IMAGE_PATH,
    LINK_START,
    LINK_TEXT,
    LINK_URL,
    LITERAL_IMAGE_START, // \![
    LITERAL_LINK_START,  // \[
    UNORDERED_LIST_ITEM,
    ORDERED_LIST_ITEM,
    INDENT,
    TEXT,
};

struct Token
{
    TokenType type;
    QString value;
};
QList<Token> tokenize(const QString &markdown)
{
    QList<Token> tokens;
    int i = 0;

    bool isInBold = false;
    bool isInItalic = false;
    bool isInUnderline = false;
    bool isInStrikethrough = false;

    while (i < markdown.length())
    {
        // Check for escaped asterisk
        if (i < markdown.length() - 1 && markdown[i] == '\\' && markdown[i + 1] == '*')
        {
            tokens.push_back({TokenType::LITERAL_ASTERISK, "*"});
            i += 2;
        }
        else if (markdown.mid(i, 2) == "**")
        {
            if (isInBold)
            {
                tokens.push_back({TokenType::BOLD_END, "**"});
                isInBold = false;
            }
            else
            {
                tokens.push_back({TokenType::BOLD_START, "**"});
                isInBold = true;
            }
            i += 2;
        }
        else if (markdown[i] == '*')
        {
            if (isInItalic)
            {
                tokens.push_back({TokenType::ITALIC_END, "*"});
                isInItalic = false;
            }
            else
            {
                tokens.push_back({TokenType::ITALIC_START, "*"});
                isInItalic = true;
            }
            i++;
        }
        else if (i < markdown.length() - 1 && markdown[i] == '\\' && markdown[i + 1] == '_')
        {
            tokens.push_back({TokenType::LITERAL_UNDERSCORE, "_"});
            i += 2;
        }
        else if (markdown[i] == '_')
        {
            if (isInUnderline)
            {
                tokens.push_back({TokenType::UNDERLINE_END, "_"});
                isInUnderline = false;
            }
            else
            {
                tokens.push_back({TokenType::UNDERLINE_START, "_"});
                isInUnderline = true;
            }
            i++;
        }
        else if (i < markdown.length() - 2 && markdown[i] == '\\' && markdown.mid(i + 1, 2) == "~~")
        {
            tokens.push_back({TokenType::LITERAL_TILDE, "~~"});
            i += 3;
        }
        else if (markdown.mid(i, 2) == "~~")
        {
            if (isInStrikethrough)
            {
                tokens.push_back({TokenType::STRIKETHROUGH_END, "~~"});
                isInStrikethrough = false;
            }
            else
            {
                tokens.push_back({TokenType::STRIKETHROUGH_START, "~~"});
                isInStrikethrough = true;
            }
            i += 2;
        }
        // Handle image: ![alt text](/path/image.jpg "title")
        else if (markdown.mid(i, 2) == "![")
        {
            tokens.push_back({TokenType::IMAGE_START, "!["});
            int altStart = i + 2; // move past the "!["
            while (i < markdown.length() && markdown[i] != ']')
            {
                i++;
            }
            tokens.push_back({TokenType::IMAGE_ALT_TEXT, markdown.mid(altStart, i - altStart)});
            i++; // Skip the ']'
            if (markdown[i] == '(')
            {
                int pathStart = i + 1;
                while (i < markdown.length() && markdown[i] != ')' && markdown[i] != ' ')
                {
                    i++;
                }
                tokens.push_back({TokenType::IMAGE_PATH, markdown.mid(pathStart, i - pathStart)});
                // Optional title (skipped for simplicity, but can be added if needed)
                while (i < markdown.length() && markdown[i] != ')')
                {
                    i++;
                }
                i++; // Skip the ')'
            }
        }
        // Handle link: [Duck Duck Go](https://duckduckgo.com)
        else if (markdown[i] == '[')
        {
            tokens.push_back({TokenType::LINK_START, "["});
            int linkTextStart = i + 1;
            while (i < markdown.length() && markdown[i] != ']')
            {
                i++;
            }
            tokens.push_back({TokenType::LINK_TEXT, markdown.mid(linkTextStart, i - linkTextStart)});
            i++; // Skip the ']'
            if (markdown[i] == '(')
            {
                int urlStart = i + 1;
                while (i < markdown.length() && markdown[i] != ')')
                {
                    i++;
                }
                tokens.push_back({TokenType::LINK_URL, markdown.mid(urlStart, i - urlStart)});
                i++; // Skip the ')'
            }
        }
        else if (i < markdown.length() - 2 && markdown[i] == '\\' && markdown.mid(i + 1, 2) == "![")
        {
            tokens.push_back({TokenType::LITERAL_IMAGE_START, "!["});
            i += 3; // Skip the '\![' sequence
        }
        // Handle escaped link: \[...]
        else if (i < markdown.length() - 1 && markdown[i] == '\\' && markdown[i + 1] == '[')
        {
            tokens.push_back({TokenType::LITERAL_LINK_START, "["});
            i += 2; // Skip the '\[' sequence
        }
        // Handle lists
        if (i < markdown.length() - 1 && markdown.mid(i, 2) == "+ " &&
            (tokens.isEmpty() || tokens.last().type == TokenType::INDENT))
        {
            tokens.push_back({TokenType::UNORDERED_LIST_ITEM, "+ "});
            i += 2;
        }
        else if (i < markdown.length() - 2 && markdown[i].isDigit() && markdown[i + 1] == '.' &&
                 markdown[i + 2] == ' ' && (tokens.isEmpty() || tokens.last().type == TokenType::INDENT))
        {
            tokens.push_back({TokenType::ORDERED_LIST_ITEM, markdown.mid(i, 3)});
            i += 3;
        }
        else if (i < markdown.length() - 3 && markdown.mid(i, 4) == "    " &&
                 (tokens.isEmpty() || tokens.last().type == TokenType::INDENT))
        {
            tokens.push_back({TokenType::INDENT, "    "});
            i += 4;
        }
        else
        {
            int start = i;
            while (i < markdown.length())
            {
                // If any markdown construct begins, stop and capture text.
                if (markdown[i] == '*' || markdown[i] == '_' || markdown.mid(i, 2) == "**" ||
                    markdown.mid(i, 2) == "~~" || markdown.mid(i, 2) == "![" || markdown[i] == '[' ||
                    (i < markdown.length() - 1 && markdown[i] == '\\' &&
                     (markdown[i + 1] == '*' || markdown[i + 1] == '[' || markdown[i + 1] == '!' ||
                      markdown[i + 1] == '~')))
                {
                    break;
                }
                i++;
            }
            if (i != start) // Only add token if there's text captured.
            {
                tokens.push_back({TokenType::TEXT, markdown.mid(start, i - start)});
            }
        }
    }

    // for any START missing an END, add an END at the end of the string
    if (isInBold)
    {
        tokens.push_back({TokenType::BOLD_END, "**"});
    }
    if (isInItalic)
    {
        tokens.push_back({TokenType::ITALIC_END, "*"});
    }
    if (isInUnderline)
    {
        tokens.push_back({TokenType::UNDERLINE_END, "_"});
    }
    if (isInStrikethrough)
    {
        tokens.push_back({TokenType::STRIKETHROUGH_END, "~~"});
    }

    // invert BOLD_END and ITALIC_END if they are next to each other so as to close properly the nested tags
    for (int i = 0; i < tokens.size() - 1; i++)
    {
        if (tokens[i].type == TokenType::BOLD_END && tokens[i + 1].type == TokenType::ITALIC_END)
        {
            std::swap(tokens[i], tokens[i + 1]);
        }
    }

    return tokens;
}

class Node
{
  public:
    virtual ~Node()
    {
    }

  public:
    std::vector<std::unique_ptr<Node>> children;
};

class TextNode : public Node
{
  public:
    QString text;
    TextNode(const QString &t) : text(t)
    {
    }
};

class BoldNode : public Node
{
};

class ItalicNode : public Node
{
};

class StrikethroughNode : public Node
{
};

class UnderlineNode : public Node
{
};

class ImageNode : public Node
{
  public:
    QString altText;
    QString imagePath;
    ImageNode(const QString &alt, const QString &path) : altText(alt), imagePath(path)
    {
    }
};

class LinkNode : public Node
{
  public:
    QString linkText;
    QString linkUrl;
    LinkNode(const QString &text, const QString &url) : linkText(text), linkUrl(url)
    {
    }
};

class ListNode : public Node
{
  public:
    enum class ListType
    {
        UNORDERED,
        ORDERED
    };

    ListType type;
    int indentLevel = 0;
};

std::unique_ptr<Node> parse(const QList<Token> &tokens, int &i, TokenType expectedEndToken = TokenType::None)
{
    int indentLevel = 0;
    std::vector<std::unique_ptr<Node>> nodes;

    while (i < tokens.size())
    {
        if (tokens[i].type == expectedEndToken)
        {
            i++; // skip the ending token and break out
            break;
        }
        else if (tokens[i].type == TokenType::BOLD_START)
        {
            i++; // Skip the token
            auto boldNode = std::make_unique<BoldNode>();
            std::unique_ptr<Node> innerNode = parse(tokens, i, TokenType::BOLD_END);
            for (auto &child : innerNode->children)
            {
                boldNode->children.push_back(std::move(child));
            }
            nodes.push_back(std::move(boldNode));
        }
        else if (tokens[i].type == TokenType::ITALIC_START)
        {
            i++; // Skip the token
            auto boldNode = std::make_unique<ItalicNode>();
            std::unique_ptr<Node> innerNode = parse(tokens, i, TokenType::ITALIC_END);
            for (auto &child : innerNode->children)
            {
                boldNode->children.push_back(std::move(child));
            }
            nodes.push_back(std::move(boldNode));
        }
        else if (tokens[i].type == TokenType::TEXT)
        {
            nodes.push_back(std::make_unique<TextNode>(tokens[i].value));
            i++;
        }
        else if (tokens[i].type == TokenType::LITERAL_ASTERISK)
        {
            nodes.push_back(std::make_unique<TextNode>(tokens[i].value)); // handle it just like plain text
            i++;
        }
        else if (tokens[i].type == TokenType::UNDERLINE_START)
        {
            i++; // Skip the token
            auto boldNode = std::make_unique<UnderlineNode>();
            std::unique_ptr<Node> innerNode = parse(tokens, i, TokenType::UNDERLINE_END);
            for (auto &child : innerNode->children)
            {
                boldNode->children.push_back(std::move(child));
            }
            nodes.push_back(std::move(boldNode));
        }

        else if (tokens[i].type == TokenType::STRIKETHROUGH_START)
        {
            i++; // Skip the token
            auto boldNode = std::make_unique<StrikethroughNode>();
            std::unique_ptr<Node> innerNode = parse(tokens, i, TokenType::STRIKETHROUGH_END);
            for (auto &child : innerNode->children)
            {
                boldNode->children.push_back(std::move(child));
            }
            nodes.push_back(std::move(boldNode));
        }
        // Handle IMAGE nodes
        else if (tokens[i].type == TokenType::IMAGE_START)
        {
            i++; // Skip IMAGE_START
            QString altText = tokens[i].value;
            i++; // Skip IMAGE_ALT_TEXT
            QString imagePath = tokens[i].value;
            i++; // Skip IMAGE_PATH
            nodes.push_back(std::make_unique<ImageNode>(altText, imagePath));
        }
        // Handle LINK nodes
        else if (tokens[i].type == TokenType::LINK_START)
        {
            i++; // Skip LINK_START
            QString linkText = tokens[i].value;
            i++; // Skip LINK_TEXT
            QString linkUrl = tokens[i].value;
            i++; // Skip LINK_URL
            nodes.push_back(std::make_unique<LinkNode>(linkText, linkUrl));
        }
        // Handle LITERAL_IMAGE_START
        else if (tokens[i].type == TokenType::LITERAL_IMAGE_START)
        {
            QString literalImageStart = tokens[i].value;
            nodes.push_back(std::make_unique<TextNode>(literalImageStart));
            i++;
        }
        // Handle LITERAL_LINK_START
        else if (tokens[i].type == TokenType::LITERAL_LINK_START)
        {
            QString literalLinkStart = tokens[i].value;
            nodes.push_back(std::make_unique<TextNode>(literalLinkStart));
            i++;
        }
        else if (tokens[i].type == TokenType::INDENT)
        {
            indentLevel++;
            i++;
            continue;
        }
        else if (tokens[i].type == TokenType::UNORDERED_LIST_ITEM || tokens[i].type == TokenType::ORDERED_LIST_ITEM)
        {
            auto listNode = std::make_unique<ListNode>();
            listNode->type = (tokens[i].type == TokenType::UNORDERED_LIST_ITEM) ? ListNode::ListType::UNORDERED
                                                                                : ListNode::ListType::ORDERED;
            listNode->indentLevel = indentLevel;
            i++; // Move past the list token
            nodes.push_back(std::move(listNode));
        }
    }

    // At the end of the function
    auto defaultNode = std::make_unique<Node>();
    for (auto &node : nodes)
    {
        defaultNode->children.push_back(std::move(node));
    }
    return defaultNode;
}

void applyFormat(QTextCursor &cursor, Node *node)
{
    if (dynamic_cast<TextNode *>(node))
    {
        TextNode *textNode = static_cast<TextNode *>(node);
        cursor.insertText(textNode->text);
    }
    else if (dynamic_cast<BoldNode *>(node))
    {
        BoldNode *boldNode = static_cast<BoldNode *>(node);
        QTextCharFormat format;
        format.setFontWeight(QFont::Bold);
        cursor.mergeCharFormat(format);
        for (const auto &child : boldNode->children)
        {
            applyFormat(cursor, child.get());
        }
        format.setFontWeight(QFont::Weight::Normal);
        cursor.mergeCharFormat(format);
    }
    else if (dynamic_cast<ItalicNode *>(node))
    {
        ItalicNode *italicNode = static_cast<ItalicNode *>(node);
        QTextCharFormat format;
        format.setFontItalic(true);
        cursor.mergeCharFormat(format);
        for (const auto &child : italicNode->children)
        {
            applyFormat(cursor, child.get());
        }
        format.setFontItalic(false);
        cursor.mergeCharFormat(format);
    }
    else if (dynamic_cast<UnderlineNode *>(node))
    {
        UnderlineNode *underlineNode = static_cast<UnderlineNode *>(node);
        QTextCharFormat format;
        format.setUnderlineStyle(QTextCharFormat::UnderlineStyle::SingleUnderline);
        format.setFontUnderline(true);
        cursor.mergeCharFormat(format);
        for (const auto &child : underlineNode->children)
        {
            applyFormat(cursor, child.get());
        }
        format.setFontUnderline(false);
        cursor.mergeCharFormat(format);
    }
    else if (dynamic_cast<StrikethroughNode *>(node))
    {
        StrikethroughNode *strikeNode = static_cast<StrikethroughNode *>(node);
        QTextCharFormat format;
        format.setFontStrikeOut(true);
        cursor.mergeCharFormat(format);
        for (const auto &child : strikeNode->children)
        {
            applyFormat(cursor, child.get());
        }
        format.setFontStrikeOut(false);
        cursor.mergeCharFormat(format);
    }
    else if (dynamic_cast<ImageNode *>(node))
    {
        ImageNode *imageNode = static_cast<ImageNode *>(node);
        QTextImageFormat imageFormat;
        imageFormat.setName(imageNode->imagePath);
        imageFormat.setToolTip(imageNode->altText);
        cursor.insertImage(imageFormat);
    }
    else if (dynamic_cast<LinkNode *>(node))
    {
        LinkNode *linkNode = static_cast<LinkNode *>(node);
        QTextCharFormat linkFormat;
        linkFormat.setAnchor(true);
        linkFormat.setAnchorHref(linkNode->linkUrl);
        cursor.insertText(linkNode->linkText, linkFormat);
    }
    else if (dynamic_cast<ListNode *>(node))
    {
        ListNode *listNode = static_cast<ListNode *>(node);

        QTextListFormat listFormat;
        if (listNode->type == ListNode::ListType::ORDERED)
        {
            listFormat.setStyle(QTextListFormat::ListDecimal);
        }
        else
        {
            listFormat.setStyle(QTextListFormat::ListDisc);
        }
        listFormat.setIndent(listNode->indentLevel);

        cursor.insertList(listFormat);
    }
    else
    {
        for (const auto &child : node->children)
        {
            applyFormat(cursor, child.get());
        }
    }
}

void fromMarkdown(QTextBlock block, const QString &markdown)
{
    const QList<Token> &tokens = tokenize(markdown);
    int i = 0;
    std::unique_ptr<Node> ast = parse(tokens, i);
    QTextCursor cursor(block);
    applyFormat(cursor, ast.get());
}

} // namespace Markdown::From
