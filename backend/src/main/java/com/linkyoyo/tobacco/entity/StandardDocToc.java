package com.linkyoyo.tobacco.entity;

import com.fasterxml.jackson.annotation.JsonFormat;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.hibernate.annotations.DynamicInsert;
import org.hibernate.annotations.DynamicUpdate;

import javax.persistence.*;
import java.io.Serializable;
import java.time.ZonedDateTime;

/**
 * 标准文档目次实体
 * 用于存储标准文档的目次信息
 */
@Entity
@Data
@AllArgsConstructor
@NoArgsConstructor
@Builder
@DynamicInsert
@DynamicUpdate
@Table(name = "standard_doc_toc")
@JsonIgnoreProperties({"hibernateLazyInitializer", "handler"})
public class StandardDocToc implements Serializable {

    @Id
    @Column(name = "id")
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Integer id; // 主键ID

    @Column(name = "standard_doc_id", nullable = false)
    private Integer standardDocId; // 标准文档ID

    @Column(name = "section_number")
    private String sectionNumber; // 章节编号，如"1"、"1.1"、"12.2"等

    @Column(name = "section_title", nullable = false)
    private String sectionTitle; // 章节标题，如"范围"、"规范性引用文件"等

    @Column(name = "page_number")
    private String pageNumber; // 页码，章节开始的页码，可以是罗马数字如"II"

    @Column(name = "display_order")
    private Integer displayOrder; // 显示顺序，在目次中的顺序

    @Column(name = "level")
    private Integer level; // 章节级别，如1表示一级标题，2表示二级标题

    @Column(name = "line_number")
    private Integer lineNumber; // 在文档中的行号

    @Column(name = "type")
    private String type; // 标题类型，如NUMBERED（编号标题）或SPECIAL（特殊标题）

    @Column(name = "original_line")
    private String originalLine; // 原始行内容，如"## 1 范围"

    @Column(name = "created_at")
    private ZonedDateTime createdAt; // 创建时间

    @Column(name = "updated_at")
    private ZonedDateTime updatedAt; // 更新时间

    /*@ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "standard_doc_id", insertable = false, updatable = false)
    private StandardDoc standardDoc; // 关联的标准文档*/
}
