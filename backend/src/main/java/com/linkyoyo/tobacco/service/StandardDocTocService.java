package com.linkyoyo.tobacco.service;

import com.linkyoyo.tobacco.entity.StandardDocToc;

import java.util.List;

/**
 * 标准文档目次服务接口
 */
public interface StandardDocTocService {

    /**
     * 根据标准文档ID获取目次列表
     * @param standardDocId 标准文档ID
     * @return 目次列表
     */
    List<StandardDocToc> getTableOfContentsByStandardDocId(Integer standardDocId);

    /**
     * 批量保存目次条目
     * @param standardDocId 标准文档ID
     * @param tocEntries 目次条目列表
     * @return 保存的记录数
     */
    Integer batchSaveTableOfContents(Integer standardDocId, List<StandardDocToc> tocEntries);

    /**
     * 手动添加目次条目
     * @param standardDocToc 目次条目
     * @return 保存的目次条目
     */
    StandardDocToc addTableOfContentsEntry(StandardDocToc standardDocToc);

    /**
     * 更新目次条目
     * @param standardDocToc 目次条目
     * @return 更新后的目次条目
     */
    StandardDocToc updateTableOfContentsEntry(StandardDocToc standardDocToc);

    /**
     * 删除目次条目
     * @param id 目次条目ID
     */
    void deleteTableOfContentsEntry(Integer id);

    /**
     * 删除标准文档的所有目次
     * @param standardDocId 标准文档ID
     */
    void deleteAllTableOfContentsByStandardDocId(Integer standardDocId);
}
