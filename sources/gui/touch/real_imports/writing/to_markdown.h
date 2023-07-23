#pragma once
#include <QTextList>
#include <QTextObject>

namespace Markdown::To
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

QList<Token> tokenizeTextBlock(const QTextBlock &block)
{
    QList<Token> tokens;
    int i = 0;

    bool isInBold = false;
    bool isInItalic = false;
    bool isInUnderline = false;
    bool isInStrikethrough = false;

    // list: unordered, ordered

    if (block.textList())
    {
        QTextList *list = block.textList();

        // indent
        int listIndent = list->format().indent();
        for (int i = 0; i < listIndent; i++)
        {
            tokens.append({TokenType::INDENT, ""});
        }

        QTextListFormat::Style style = list->format().style();
        if (style == QTextListFormat::ListDisc)
        {
            // unordered
            tokens.append({TokenType::UNORDERED_LIST_ITEM, ""});
        }
        else if (style == QTextListFormat::ListDecimal)
        {
            // ordered
            tokens.append({TokenType::ORDERED_LIST_ITEM, ""});
        }
    }

    QTextBlock::iterator it;

    for (it = block.begin(); !(it.atEnd()); ++it)
    {
        const QTextFragment &currentFragment = it.fragment();
        const QTextCharFormat &charFormat = currentFragment.charFormat();
        QList<Token> formatEndTokens;

        if (currentFragment.isValid())
        {
            // bold
            if (charFormat.fontWeight() == QFont::Bold)
            {
                isInBold = true;
                tokens.append({TokenType::BOLD_START, ""});
            }
            else if (charFormat.fontWeight() == QFont::Normal && isInBold)
            {
                isInBold = false;
                formatEndTokens.append({TokenType::BOLD_END, ""});
            }

            // italic
            if (charFormat.fontItalic())
            {
                isInItalic = true;
                tokens.append({TokenType::ITALIC_START, ""});
            }
            else if (!charFormat.fontItalic() && isInItalic)
            {
                isInItalic = false;
                formatEndTokens.append({TokenType::ITALIC_END, ""});
            }

            // underline
            if (charFormat.fontUnderline())
            {
                isInUnderline = true;
                tokens.append({TokenType::UNDERLINE_START, ""});
            }
            else if (!charFormat.fontUnderline() && isInUnderline)
            {
                isInUnderline = false;
                formatEndTokens.append({TokenType::UNDERLINE_END, ""});
            }

            // strikethrough
            if (charFormat.fontStrikeOut())
            {
                isInStrikethrough = true;
                tokens.append({TokenType::STRIKETHROUGH_START, ""});
            }
            else if (!charFormat.fontStrikeOut() && isInStrikethrough)
            {
                isInStrikethrough = false;
                formatEndTokens.append({TokenType::STRIKETHROUGH_END, ""});
            }

            // bold end, italic end, underline end, strikethrough end, if found next to each other in this order
            // , whatever the number of them, must have their order inverted
            // this is because the last one to be opened must be the first one to be closed

            // reverse the order of formatEndTokens
            std::reverse(formatEndTokens.begin(), formatEndTokens.end());

            // append the reversed tokens
            tokens.append(formatEndTokens);

            // images
            if (charFormat.isImageFormat())
            {
                tokens.append({TokenType::IMAGE_START, ""});
                tokens.append({TokenType::IMAGE_ALT_TEXT, charFormat.toolTip()});
                tokens.append({TokenType::IMAGE_PATH, charFormat.toImageFormat().name()});
            }

            // links
            if (charFormat.isAnchor())
            {
                tokens.append({TokenType::LINK_START, ""});
                tokens.append({TokenType::LINK_TEXT, currentFragment.text()});
                tokens.append({TokenType::LINK_URL, charFormat.anchorHref()});
            }

            // text
            if (!charFormat.isAnchor() && !charFormat.isImageFormat())
            {
                tokens.append({TokenType::TEXT, currentFragment.text()});
            }
        }
    }

    // for any START missing an END, add an END at the end of the string (reversed)
    if (isInStrikethrough)
    {
        tokens.push_back({TokenType::STRIKETHROUGH_END, ""});
    }
    if (isInUnderline)
    {
        tokens.push_back({TokenType::UNDERLINE_END, ""});
    }
    if (isInItalic)
    {
        tokens.push_back({TokenType::ITALIC_END, ""});
    }
    if (isInBold)
    {
        tokens.push_back({TokenType::BOLD_END, ""});
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
            i++; // Skip the bold token
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
            auto italicdNode = std::make_unique<ItalicNode>();
            std::unique_ptr<Node> innerNode = parse(tokens, i, TokenType::ITALIC_END);
            for (auto &child : innerNode->children)
            {
                italicdNode->children.push_back(std::move(child));
            }
            nodes.push_back(std::move(italicdNode));
        }
        else if (tokens[i].type == TokenType::TEXT)
        {
            // replace the charaters *, _, ~, ![, [ and \ with their escaped version
            QString text = tokens[i].value;
            text.replace("\\", "\\\\");
            text.replace("*", "\\*");
            text.replace("_", "\\_");
            text.replace("~", "\\~");
            text.replace("![", "\\![");
            text.replace("[", "\\[");

            nodes.push_back(std::make_unique<TextNode>(text));
            i++;
        }
        else if (tokens[i].type == TokenType::UNDERLINE_START)
        {
            i++; // Skip the token
            auto underlineNode = std::make_unique<UnderlineNode>();
            std::unique_ptr<Node> innerNode = parse(tokens, i, TokenType::UNDERLINE_END);
            for (auto &child : innerNode->children)
            {
                underlineNode->children.push_back(std::move(child));
            }
            nodes.push_back(std::move(underlineNode));
        }

        else if (tokens[i].type == TokenType::STRIKETHROUGH_START)
        {
            i++; // Skip the token
            auto strikeNode = std::make_unique<StrikethroughNode>();
            std::unique_ptr<Node> innerNode = parse(tokens, i, TokenType::STRIKETHROUGH_END);
            for (auto &child : innerNode->children)
            {
                strikeNode->children.push_back(std::move(child));
            }
            nodes.push_back(std::move(strikeNode));
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

void applyFormatToMarkdown(QString &markdownstring, int &i, Node *node)
{
    if (auto textNode = dynamic_cast<TextNode *>(node))
    {
        markdownstring.append(textNode->text);
        i += textNode->text.size();
    }
    else if (auto boldNode = dynamic_cast<BoldNode *>(node))
    {
        markdownstring.append("**");
        i += 2;
        for (auto &child : boldNode->children)
        {
            applyFormatToMarkdown(markdownstring, i, child.get());
        }
        markdownstring.append("**");
        i += 2;
    }
    else if (auto italicNode = dynamic_cast<ItalicNode *>(node))
    {
        markdownstring.append("*");
        i += 1;
        for (auto &child : italicNode->children)
        {
            applyFormatToMarkdown(markdownstring, i, child.get());
        }
        markdownstring.append("*");
        i += 1;
    }
    else if (auto strikethroughNode = dynamic_cast<StrikethroughNode *>(node))
    {
        markdownstring.append("~~");
        i += 2;
        for (auto &child : strikethroughNode->children)
        {
            applyFormatToMarkdown(markdownstring, i, child.get());
        }
        markdownstring.append("~~");
        i += 2;
    }
    else if (auto underlineNode = dynamic_cast<UnderlineNode *>(node))
    {
        markdownstring.append("_");
        i += 1;
        for (auto &child : underlineNode->children)
        {
            applyFormatToMarkdown(markdownstring, i, child.get());
        }
        markdownstring.append("_");
        i += 1;
    }
    else if (auto imageNode = dynamic_cast<ImageNode *>(node))
    {
        markdownstring.append("![");
        i += 2;
        markdownstring.append(imageNode->altText);
        i += imageNode->altText.size();
        markdownstring.append("](");
        i += 2;
        markdownstring.append(imageNode->imagePath);
        i += imageNode->imagePath.size();
        markdownstring.append(")");
        i += 1;
    }
    else if (auto linkNode = dynamic_cast<LinkNode *>(node))
    {
        markdownstring.append("[");
        i += 1;
        markdownstring.append(linkNode->linkText);
        i += linkNode->linkText.size();
        markdownstring.append("](");
        i += 2;
        markdownstring.append(linkNode->linkUrl);
        i += linkNode->linkUrl.size();
        markdownstring += ")";
        i += 1;
    }
    else
    {
        for (const auto &child : node->children)
        {
            applyFormatToMarkdown(markdownstring, i, child.get());
        }
    }
}

QString toMarkdown(const QTextBlock &block)
{
    const QList<Token> &tokens = tokenizeTextBlock(block);
    int i = 0;
    std::unique_ptr<Node> ast = parse(tokens, i);
    QString markdownString;
    int stringIndex = 0;
    applyFormatToMarkdown(markdownString, stringIndex, ast.get());

    return markdownString;
}

} // namespace Markdown::To
