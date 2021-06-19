<p align="center">
	<h3 align="center">Dorea DB</h3>
	<p align="center">
    <a href="https://github.com/mrxiaozhuox/Dorea/actions">
    	<img alt="Build" src="https://img.shields.io/github/workflow/status/mrxiaozhuox/Dorea/Rust?style=for-the-badge" />
    </a>
    <a href="https://github.com/mrxiaozhuox/Dorea/blob/master/LICENSE">
      <img alt="GitHub" src="https://img.shields.io/github/license/mrxiaozhuox/Dorea?style=for-the-badge">
    </a>
    <a href="https://github.com/mrxiaozhuox/Dorea/blob/master/LICENSE">
			<img alt="Code" src="https://img.shields.io/github/languages/code-size/mrxiaozhuox/Dorea?style=for-the-badge">
    </a>
	</p>
</p>



#### 服务器运行

当服务器被启动，系统将自动从默认数据库中读取最大数量的数据进入内存（可自定义配置）

默认数据库的优先级最高，将常用且重要的数据存放在默认数据库可保证数据加载效率与安全性。



#### 缓存淘汰机制

本项目采用 **LRU** 的缓存淘汰机制，即 **(Least recently used)**

当一条数据长时间未被缓存击中，则将由新获取的缓存替代它。

此方法的命中率一般，但是可以满足基本需求。



#### 数据储存机制

内存缓存 **(Memory Cache)** 采用了基本的 HashMap 实现，保证了基本查询的快捷。

**持久化数据储存** 则是使用了类似于 BTree 的储存方法，方便了数据的模糊查找。

数据库会维护一个 **.idx** 文件，文件将提高初始化时数据的定位效率，尽量排除使用遍历等方法。



#### 类型自动推导

插入数据时会自动对你输入的类型进行简单的解析。并分成以下几类：

- String - 字符串
- Number - 数字 ( **isize** )
- Boolean - 布尔值
- Dict - 字典表



#### 系统日志记录

数据库内置完善的数据记录系统，对于每一次增删改查都有完整的日志记录。
